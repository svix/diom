use std::{fs, str::FromStr, time::Instant};

use crate::{
    cfg::Configuration as AppConfig,
    core::{INTERNAL_NAMESPACE, cluster::RaftState},
    v1::endpoints::{
        auth_token::AuthTokenCreateNamespaceIn, cache::CacheCreateNamespaceIn,
        idempotency::IdempotencyCreateNamespaceIn, kv::KvCreateNamespaceIn,
        msgs::MsgNamespaceCreateIn, rate_limit::RateLimitCreateNamespaceIn,
    },
};
use anyhow::{Context, bail};
use coyote_msgs::entities::Retention;
use coyote_namespace::{DEFAULT_NAMESPACE_NAME, entities::EvictionPolicy};

#[derive(Debug)]
enum BootstrapCommand {
    Kv(KvCreateNamespaceIn),
    Cache(CacheCreateNamespaceIn),
    Idempotency(IdempotencyCreateNamespaceIn),
    RateLimit(RateLimitCreateNamespaceIn),
    Msgs(MsgNamespaceCreateIn),
    AuthToken(AuthTokenCreateNamespaceIn),
}

impl BootstrapCommand {
    async fn apply(self, raft_state: &RaftState) -> anyhow::Result<()> {
        match self {
            BootstrapCommand::Kv(v) => {
                tracing::debug!(name = v.name, "bootstrapping kv");
                raft_state
                    .client_write(coyote_kv::operations::CreateKvOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Cache(v) => {
                tracing::debug!(name = v.name, "bootstrapping cache");
                raft_state
                    .client_write(coyote_cache::operations::CreateCacheOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Idempotency(v) => {
                tracing::debug!(name = v.name, "bootstrapping idempotency");
                raft_state
                    .client_write(
                        coyote_idempotency::operations::CreateIdempotencyOperation::from(v),
                    )
                    .await?;
            }
            BootstrapCommand::RateLimit(v) => {
                tracing::debug!(name = v.name, "bootstrapping rate-limit");
                raft_state
                    .client_write(coyote_rate_limit::operations::CreateRateLimitOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Msgs(v) => {
                tracing::debug!(name = v.name, "bootstrapping msgs");
                raft_state
                    .client_write(coyote_msgs::operations::CreateNamespaceOperation::from(v))
                    .await?;
            }
            BootstrapCommand::AuthToken(v) => {
                tracing::debug!(name = v.name, "bootstrapping auth_token");
                raft_state
                    .client_write(
                        coyote_auth_token::operations::CreateAuthTokenNamespaceOperation::from(v),
                    )
                    .await?;
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        match self {
            BootstrapCommand::Kv(v) => &v.name,
            BootstrapCommand::Cache(v) => &v.name,
            BootstrapCommand::Idempotency(v) => &v.name,
            BootstrapCommand::RateLimit(v) => &v.name,
            BootstrapCommand::Msgs(v) => &v.name,
            BootstrapCommand::AuthToken(v) => &v.name,
        }
    }
}

impl FromStr for BootstrapCommand {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> anyhow::Result<Self> {
        let (module, rest) = line.split_once(char::is_whitespace).with_context(|| {
            format!("expected '<module> namespace create <json>', got: {line:?}")
        })?;
        let rest = rest.trim_start();

        let (resource, rest) = rest
            .split_once(char::is_whitespace)
            .with_context(|| format!("expected 'namespace create <json>', got: {rest:?}"))?;
        if resource != "namespace" {
            bail!("expected 'namespace', got {resource:?}");
        }
        let rest = rest.trim_start();

        let (action, json_str) = rest
            .split_once(char::is_whitespace)
            .with_context(|| format!("expected 'create <json>', got: {rest:?}"))?;
        if action != "create" {
            bail!("expected 'create', got {action:?}");
        }
        let json_str = json_str.trim();

        match module {
            "kv" => Ok(BootstrapCommand::Kv(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for kv namespace: {json_str:?}"))?,
            )),
            "cache" => Ok(BootstrapCommand::Cache(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for cache namespace: {json_str:?}"))?,
            )),
            "idempotency" => Ok(BootstrapCommand::Idempotency(
                serde_json::from_str(json_str).with_context(|| {
                    format!("invalid JSON for idempotency namespace: {json_str:?}")
                })?,
            )),
            "rate-limit" => Ok(BootstrapCommand::RateLimit(
                serde_json::from_str(json_str).with_context(|| {
                    format!("invalid JSON for rate-limit namespace: {json_str:?}")
                })?,
            )),
            "msgs" => Ok(BootstrapCommand::Msgs(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for msgs namespace: {json_str:?}"))?,
            )),
            _ => bail!("unknown module {module:?}"),
        }
    }
}

fn parse_bootstrap(content: &str) -> anyhow::Result<Vec<BootstrapCommand>> {
    let mut commands = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cmd = line
            .parse::<BootstrapCommand>()
            .with_context(|| format!("error on line {}", i + 1))?;
        commands.push(cmd);
    }
    Ok(commands)
}

fn ensure_defaults(commands: &mut Vec<BootstrapCommand>) {
    macro_rules! ensure_default {
        ($variant:ident, $constructor:expr) => {
            if !commands.iter().any(|c| {
                matches!(c, BootstrapCommand::$variant(_)) && c.name() == DEFAULT_NAMESPACE_NAME
            }) {
                commands.insert(0, $constructor);
            }
        };
    }

    ensure_default!(
        AuthToken,
        BootstrapCommand::AuthToken(AuthTokenCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
        })
    );
    commands.insert(
        0,
        BootstrapCommand::AuthToken(AuthTokenCreateNamespaceIn {
            name: INTERNAL_NAMESPACE.to_string(),
            max_storage_bytes: None,
        }),
    );
    ensure_default!(
        Msgs,
        BootstrapCommand::Msgs(MsgNamespaceCreateIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            retention: Retention::default(),
        })
    );
    ensure_default!(
        RateLimit,
        BootstrapCommand::RateLimit(RateLimitCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
        })
    );
    ensure_default!(
        Idempotency,
        BootstrapCommand::Idempotency(IdempotencyCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
        })
    );
    ensure_default!(
        Cache,
        BootstrapCommand::Cache(CacheCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
            eviction_policy: EvictionPolicy::NoEviction,
        })
    );
    ensure_default!(
        Kv,
        BootstrapCommand::Kv(KvCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
        })
    );
}

fn load_commands(
    config_path: Option<&str>,
    config_content: Option<&str>,
) -> anyhow::Result<Vec<BootstrapCommand>> {
    let content = match (config_content, config_path) {
        (Some(content), _) => content.to_owned(),
        (None, Some(path)) => fs::read_to_string(path)
            .with_context(|| format!("opening bootstrap config {path:?}"))?,
        (None, None) => String::new(),
    };
    let mut commands = parse_bootstrap(&content).context("parsing bootstrap config")?;
    ensure_defaults(&mut commands);
    Ok(commands)
}

pub async fn run(app_config: AppConfig, raft_state: RaftState) -> anyhow::Result<()> {
    let t = Instant::now();
    // FIXME: Do something smarter here:
    let mut retries = 100;
    let shutdown = crate::shutting_down_token();
    while !raft_state.is_up().await && retries > 0 {
        retries -= 1;
        if shutdown
            .run_until_cancelled(tokio::time::sleep(std::time::Duration::from_millis(100)))
            .await
            .is_none()
        {
            anyhow::bail!("shut down before bootstrap finished");
        }
    }

    let commands = load_commands(
        app_config.bootstrap_cfg_path.as_deref(),
        app_config.bootstrap_cfg.as_deref(),
    )?;

    tracing::debug!(
        num_commands = commands.len(),
        persistent_db = ?app_config.persistent_db,
        ephemeral_db = ?app_config.ephemeral_db,
        "Starting bootstrapping."
    );

    for cmd in commands {
        cmd.apply(&raft_state).await?;
    }

    tracing::info!(
        duration_ms = (Instant::now() - t).as_millis(),
        "Finished bootstrapping."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv_defaults() {
        let cmd: BootstrapCommand = r#"kv namespace create {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Kv(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name, "myns");
        assert_eq!(v.max_storage_bytes, None);
    }

    #[test]
    fn kv_with_options() {
        let cmd: BootstrapCommand =
            r#"kv namespace create {"name":"myns","max_storage_bytes":1024}"#
                .parse()
                .unwrap();
        let BootstrapCommand::Kv(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name, "myns");
        assert_eq!(v.max_storage_bytes.unwrap().get(), 1024);
    }

    #[test]
    fn cache_defaults() {
        let cmd: BootstrapCommand = r#"cache namespace create {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Cache(v) = cmd else {
            panic!()
        };
        assert_eq!(v.eviction_policy, EvictionPolicy::NoEviction);
    }

    #[test]
    fn cache_with_options() {
        let cmd: BootstrapCommand =
            r#"cache namespace create {"name":"myns","eviction_policy":"LeastRecentlyUsed"}"#
                .parse()
                .unwrap();
        let BootstrapCommand::Cache(v) = cmd else {
            panic!()
        };
        assert_eq!(v.eviction_policy, EvictionPolicy::LeastRecentlyUsed);
    }

    #[test]
    fn idempotency() {
        let cmd: BootstrapCommand =
            r#"idempotency namespace create {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Idempotency(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name, "myns");
    }

    #[test]
    fn rate_limit() {
        let cmd: BootstrapCommand =
            r#"rate-limit namespace create {"name":"myns"}"#.parse().unwrap();
        assert!(matches!(cmd, BootstrapCommand::RateLimit(v) if &v.name == "myns"));
    }

    #[test]
    fn msgs_defaults() {
        let cmd: BootstrapCommand = r#"msgs namespace create {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Msgs(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name, "myns");
        assert_eq!(v.retention, Retention::default());
    }

    #[test]
    fn msgs_with_options() {
        let cmd: BootstrapCommand =
            r#"msgs namespace create {"name":"myns","retention":{"ms":60000,"bytes":500}}"#
                .parse()
                .unwrap();
        let BootstrapCommand::Msgs(v) = cmd else {
            panic!()
        };
        assert_eq!(v.retention.ms.get(), 60000);
        assert_eq!(v.retention.bytes.get(), 500);
    }

    #[test]
    fn too_few_tokens() {
        assert!("kv namespace create".parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn wrong_resource() {
        assert!(r#"kv config create {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn wrong_action() {
        assert!(r#"kv namespace update {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn unknown_module() {
        assert!(r#"blob namespace create {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn invalid_json() {
        assert!(
            "kv namespace create not-json"
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn missing_name_field() {
        assert!(r#"kv namespace create {}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn invalid_eviction_policy() {
        assert!(
            r#"cache namespace create {"name":"myns","eviction_policy":"random"}"#
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn zero_max_storage_bytes_rejected() {
        assert!(
            r#"kv namespace create {"name":"myns","max_storage_bytes":0}"#
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn skips_blank_lines_and_comments() {
        let input = r#"
            # this is a comment
            kv namespace create {"name":"foo"}

            # another comment
            cache namespace create {"name":"bar"}
        "#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 2);
        assert!(matches!(&cmds[0], BootstrapCommand::Kv(v) if v.name == "foo"));
        assert!(matches!(&cmds[1], BootstrapCommand::Cache(v) if v.name == "bar"));
    }

    #[test]
    fn parse_error_includes_line_number() {
        let input = "kv namespace create {\"name\":\"foo\"}\nkv namespace create not-json\n";
        let err = parse_bootstrap(input).unwrap_err();
        assert!(err.to_string().contains("line 2"), "error was: {err}");
    }

    #[test]
    fn empty_input_produces_no_commands() {
        assert!(parse_bootstrap("").unwrap().is_empty());
    }

    #[test]
    fn ensure_defaults_injects_all_five_when_empty() {
        let mut cmds = vec![];
        ensure_defaults(&mut cmds);
        // Make sure at least the original five are there
        assert!(cmds.len() >= 5);
        assert!(
            cmds.iter()
                .any(|c| matches!(c, BootstrapCommand::Kv(v) if v.name == DEFAULT_NAMESPACE_NAME))
        );
        assert!(
            cmds.iter().any(
                |c| matches!(c, BootstrapCommand::Cache(v) if v.name == DEFAULT_NAMESPACE_NAME)
            )
        );
        assert!(cmds.iter().any(
            |c| matches!(c, BootstrapCommand::Idempotency(v) if v.name == DEFAULT_NAMESPACE_NAME)
        ));
        assert!(cmds.iter().any(
            |c| matches!(c, BootstrapCommand::RateLimit(v) if v.name == DEFAULT_NAMESPACE_NAME)
        ));
        assert!(
            cmds.iter().any(
                |c| matches!(c, BootstrapCommand::Msgs(v) if v.name == DEFAULT_NAMESPACE_NAME)
            )
        );
    }

    #[test]
    fn ensure_defaults_does_not_duplicate_existing_default() {
        let mut cmds = vec![BootstrapCommand::Kv(KvCreateNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.to_string(),
            max_storage_bytes: None,
        })];
        ensure_defaults(&mut cmds);
        let kv_defaults: Vec<_> = cmds
            .iter()
            .filter(|c| matches!(c, BootstrapCommand::Kv(v) if v.name == DEFAULT_NAMESPACE_NAME))
            .collect();
        assert_eq!(kv_defaults.len(), 1);
    }

    #[test]
    fn ensure_defaults_does_not_suppress_non_default_namespaces() {
        let mut cmds = vec![BootstrapCommand::Kv(KvCreateNamespaceIn {
            name: "other".to_string(),
            max_storage_bytes: None,
        })];
        ensure_defaults(&mut cmds);
        let kv_count = cmds
            .iter()
            .filter(|c| matches!(c, BootstrapCommand::Kv(_)))
            .count();
        assert_eq!(kv_count, 2);
    }
}
