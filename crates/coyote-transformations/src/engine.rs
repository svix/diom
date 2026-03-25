use std::time::{Duration, Instant};

use rand::RngCore;
use rquickjs::{
    CatchResultExt, Ctx, JsLifetime, Promise, Value, class::Trace, context::EvalOptions,
    function::Rest,
};
use thiserror::Error;
use tracing::{Instrument, field::Empty};

/// Max heap allocations: 5MiB
const MAX_RAM_BYTES: usize = 5_242_880;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("internal error setting up the JavaScript interpreter: {0}")]
    InternalError(rquickjs::Error),
    #[error("unable to parse input into QuickJS: {0}")]
    InvalidInputEncoding(rquickjs::Error),
    #[error("unable to serialize output: {0}")]
    InvalidOutputDecoding(rquickjs::Error),
    #[error("script failed to return an output")]
    NoOutput,
    #[error("exception raised by JavaScript code: {message:?} {}", stack.as_deref().unwrap_or(""))]
    ExecutionException {
        message: String,
        stack: Option<String>,
    },
    #[error("Evaluating the top-level module failed to return a valid promise")]
    InvalidPromise,
    #[error("maximum processing time exceeded")]
    ProcessTimeout,
    #[error("maximum RAM use exceeded")]
    OutOfMemory,
}

impl From<ScriptError> for coyote_error::Error {
    fn from(err: ScriptError) -> Self {
        match err {
            ScriptError::InternalError(_) | ScriptError::InvalidPromise => {
                coyote_error::Error::internal(err)
            }
            ScriptError::InvalidInputEncoding(_)
            | ScriptError::InvalidOutputDecoding(_)
            | ScriptError::NoOutput => coyote_error::Error::bad_request("content_validation", err),
            ScriptError::ExecutionException { .. } | ScriptError::ProcessTimeout => {
                coyote_error::Error::bad_request("execution_timeout", err)
            }
            ScriptError::OutOfMemory => coyote_error::Error::bad_request("out_of_memory", err),
        }
    }
}

#[derive(Clone, Trace, JsLifetime)]
#[rquickjs::class(frozen)]
struct CoyoteCrypto {}

#[rquickjs::methods(rename_all = "camelCase")]
impl CoyoteCrypto {
    #[qjs(rename = "randomUUID")]
    pub(crate) fn random_uuid(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub(crate) fn get_random_bytes<'js>(
        &self,
        ctx: Ctx<'js>,
        num_bytes: usize,
    ) -> rquickjs::Result<rquickjs::TypedArray<'js, u8>> {
        let mut vec = vec![0u8; num_bytes];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut vec);
        rquickjs::TypedArray::new(ctx, vec)
    }

    pub(crate) fn array_to_hex<'js>(&self, bytes: rquickjs::TypedArray<'js, u8>) -> String {
        hex::encode(bytes)
    }
}

#[derive(Clone, Trace, JsLifetime)]
#[rquickjs::class(frozen)]
struct Console {}

#[rquickjs::methods]
impl Console {
    fn debug(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("debug called from script");
        Ok(())
    }

    fn log(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("log called from script");
        Ok(())
    }

    fn warn(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("warn called from script");
        Ok(())
    }

    fn error(&self, _values: Rest<Value<'_>>) -> rquickjs::Result<()> {
        tracing::debug!("error called from script");
        Ok(())
    }
}

fn handle_caught_error(e: rquickjs::CaughtError<'_>) -> ScriptError {
    match e {
        rquickjs::CaughtError::Exception(exc) => {
            if exc.message().as_deref() == Some("interrupted") {
                return ScriptError::ProcessTimeout;
            }
            if exc.message().as_deref() == Some("out of memory") {
                ScriptError::OutOfMemory
            } else {
                ScriptError::ExecutionException {
                    message: exc
                        .message()
                        .unwrap_or_else(|| "<unknown error>".to_string()),
                    stack: exc.stack(),
                }
            }
        }
        rquickjs::CaughtError::Error(err) => ScriptError::ExecutionException {
            message: format!("{err:?}"),
            stack: None,
        },
        rquickjs::CaughtError::Value(v) => {
            // OOM is represented by raising `null` as an exception. who knows why.
            if v.is_null() {
                ScriptError::OutOfMemory
            } else {
                ScriptError::ExecutionException {
                    message: format!("{v:?}"),
                    stack: None,
                }
            }
        }
    }
}

#[tracing::instrument(skip_all, level = "debug", fields(runtime_us = Empty, max_memory_bytes = Empty))]
pub async fn run_script(
    script: impl Into<String>,
    payload_json: &str,
    max_duration: Duration,
) -> Result<String, ScriptError> {
    let start = Instant::now();
    let result = run_script_inner(script.into(), payload_json, max_duration)
        .instrument(tracing::Span::current())
        .await;
    if let Err(e) = &result {
        tracing::warn!(
            err = ?e,
            runtime_us = start.elapsed().as_micros(),
            "error executing transformation"
        );
    }
    result
}

async fn run_script_inner(
    script: String,
    payload_json: &str,
    max_duration: Duration,
) -> Result<String, ScriptError> {
    let runtime = rquickjs::AsyncRuntime::new().map_err(ScriptError::InternalError)?;
    runtime.set_memory_limit(MAX_RAM_BYTES).await;

    let start = Instant::now();
    runtime
        .set_interrupt_handler(Some(Box::new(move || start.elapsed() > max_duration)))
        .await;

    let context = rquickjs::AsyncContext::full(&runtime)
        .await
        .map_err(ScriptError::InternalError)?;

    context
        .with(|ctx| {
            let globals = ctx.globals();
            globals
                .set("crypto", CoyoteCrypto {})
                .map_err(ScriptError::InternalError)?;
            globals
                .set("console", Console {})
                .map_err(ScriptError::InternalError)?;
            globals
                .set("script_input", payload_json)
                .map_err(ScriptError::InternalError)?;
            Ok(())
        })
        .await?;

    let mut full_script = script;
    let value = rquickjs::async_with!(context => |ctx| {
        let mut options = EvalOptions::default();
        options.global = true;
        options.strict = false;
        options.promise = true;

        full_script.push_str(";\n handler(JSON.parse(script_input))");
        let value = ctx
            .eval_with_options::<Promise<'_>, String>(full_script, options)
            .catch(&ctx)
            .map_err(handle_caught_error)?
            .into_future::<Value<'_>>()
            .await
            .catch(&ctx)
            .map_err(handle_caught_error)?;
        // https://github.com/DelSkayn/rquickjs/issues/360
        let object = value.as_object().ok_or(ScriptError::InvalidPromise)?;
        let value: Value<'_> = object.get("value").map_err(|_| ScriptError::InvalidPromise)?;

        ctx.json_stringify(value)
            .and_then(|opt| match opt {
                Some(val) => val.to_string().map(Some),
                None => Ok(None),
            })
            .map_err(ScriptError::InvalidOutputDecoding)?
            .ok_or(ScriptError::NoOutput)
    })
    .await?;

    let span = tracing::Span::current();
    span.record("runtime_us", start.elapsed().as_micros());

    let usage = runtime.memory_usage().await;
    span.record("max_memory_bytes", usage.memory_used_size);

    Ok(value)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use std::{
        str::FromStr,
        time::{Duration, Instant},
    };

    use super::{ScriptError, run_script};
    use tracing_test::traced_test;

    async fn exec(script: &str, data: &str) -> Result<String, ScriptError> {
        run_script(script, data, Duration::from_millis(10)).await
    }

    #[tokio::test]
    async fn test_random_uuid() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) { return globalThis.crypto.randomUUID(); }"#,
            "null",
        )
        .await?;
        // response is a JSON string, e.g. `"xxxxxxxx-..."`
        let s: String = serde_json::from_str(&response)?;
        let parsed = uuid::Uuid::from_str(&s)?;
        assert_eq!(parsed.get_version_num(), 4);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_random_bytes() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                return { "small": Array.from(globalThis.crypto.getRandomBytes(32)) }
            }"#,
            "null",
        )
        .await?;
        let parsed: Value = serde_json::from_str(&response)?;
        let Some(Value::Array(ints)) = parsed.pointer("/small") else {
            anyhow::bail!("Unexpected return type, got {parsed:?}");
        };
        assert_eq!(ints.len(), 32);
        assert!(ints.iter().all(|i| matches!(i, Value::Number(_))));
        let numbers = ints.iter().map(|i| i.as_i64().unwrap()).collect::<Vec<_>>();
        assert!(numbers.iter().all(|i| (0..=256).contains(i)));
        assert!(numbers.iter().any(|i| *i != 0 && *i != 256));
        Ok(())
    }

    #[tokio::test]
    async fn test_array_to_hex() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                const array = new Uint8Array([222, 173, 190, 239]);
                return globalThis.crypto.arrayToHex(array);
            }"#,
            "null",
        )
        .await?;
        assert_eq!(response, r#""deadbeef""#);
        Ok(())
    }

    #[tokio::test]
    async fn test_slow_gets_killed() -> anyhow::Result<()> {
        let start = Instant::now();
        let response = exec(
            r#"function handler(input) {
                var j = 0;
                for (var i = 0 ; i < 10000000000; ++i) {}
            }"#,
            "null",
        )
        .await;
        let dur = start.elapsed();
        let Err(ScriptError::ProcessTimeout) = response else {
            anyhow::bail!("expected an interrupted exception; got {response:?}");
        };
        assert!(dur >= Duration::from_millis(9));
        assert!(dur <= Duration::from_millis(100));
        Ok(())
    }

    #[tokio::test]
    async fn test_leaky_gets_killed() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                let array = [1, 2, 3, 4];
                for (var i = 0 ; i < 10000; ++i) {
                    array.push(JSON.parse(JSON.stringify(array)));
                }
                return 1;
            }"#,
            "null",
        )
        .await;
        let Err(ScriptError::OutOfMemory) = response else {
            anyhow::bail!("expected an OOM error; got {response:?}");
        };
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn test_console_log_goes_to_tracing() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                console.log("logged at info");
                console.error("logged at error");
                return 42
            }"#,
            "null",
        )
        .await?;
        assert_eq!(response, "42");
        assert!(logs_contain("log called from script"));
        assert!(logs_contain("error called from script"));
        Ok(())
    }

    #[tokio::test]
    async fn test_allows_globals() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                newOutput = 42;
                return newOutput;
            }"#,
            "null",
        )
        .await?;
        assert_eq!(response, "42");
        Ok(())
    }

    #[tokio::test]
    async fn test_error_message() -> anyhow::Result<()> {
        let response = exec(
            r#"function handler(input) {
                input.map((x) => x + 1);
                return input;
            }"#,
            "{}",
        )
        .await;
        let err = response.expect_err("should return an error");
        let ScriptError::ExecutionException { message, stack } = err else {
            panic!("Unexpected error")
        };
        assert_eq!(message, "not a function");
        assert_eq!(
            stack.as_deref(),
            Some("    at handler (eval_script:2:23)\n    at <eval> (eval_script:5:21)\n")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_payload_is_accessible() -> anyhow::Result<()> {
        let result = run_script(
            r#"function handler(input) { return input.x + 1; }"#,
            r#"{"x":41}"#,
            Duration::from_millis(100),
        )
        .await?;
        assert_eq!(result, "42");
        Ok(())
    }

    #[tokio::test]
    async fn test_return_map() -> anyhow::Result<()> {
        let result = run_script(
            r#"function handler(input) { return {"foo": {"bar": 123, "x": true}}; }"#,
            r#"{"x":41}"#,
            Duration::from_millis(100),
        )
        .await?;
        assert_eq!(result, "{\"foo\":{\"bar\":123,\"x\":true}}");
        Ok(())
    }
}
