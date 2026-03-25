use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    thread::available_parallelism,
    time::Duration,
};

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::{Mutex, Semaphore, mpsc, oneshot},
    time::timeout,
};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::engine::ScriptError;

// Types for the std::io IPC protocol

#[derive(Serialize, Deserialize)]
struct WorkerRequestMsg {
    id: u64,
    script: String,
    input: String,
    timeout_ms: u64,
}

#[derive(Serialize, Deserialize)]
struct WorkerResponseMsg {
    id: u64,
    response: WorkerResponse,
}

#[derive(Serialize, Deserialize)]
enum WorkerResponse {
    Ok(String),
    Err(WorkerError),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
enum WorkerError {
    InternalError {
        message: String,
    },
    InvalidInputEncoding {
        message: String,
    },
    InvalidOutputDecoding {
        message: String,
    },
    NoOutput,
    ExecutionException {
        message: String,
        stack: Option<String>,
    },
    InvalidPromise,
    ProcessTimeout,
    OutOfMemory,
}

impl From<ScriptError> for WorkerError {
    fn from(e: ScriptError) -> Self {
        match e {
            ScriptError::InternalError(err) => WorkerError::InternalError {
                message: err.to_string(),
            },
            ScriptError::InvalidInputEncoding(err) => WorkerError::InvalidInputEncoding {
                message: err.to_string(),
            },
            ScriptError::InvalidOutputDecoding(err) => WorkerError::InvalidOutputDecoding {
                message: err.to_string(),
            },
            ScriptError::NoOutput => WorkerError::NoOutput,
            ScriptError::ExecutionException { message, stack } => {
                WorkerError::ExecutionException { message, stack }
            }
            ScriptError::InvalidPromise => WorkerError::InvalidPromise,
            ScriptError::ProcessTimeout => WorkerError::ProcessTimeout,
            ScriptError::OutOfMemory => WorkerError::OutOfMemory,
        }
    }
}

impl From<WorkerError> for ScriptError {
    fn from(e: WorkerError) -> Self {
        match e {
            WorkerError::InternalError { .. } => {
                ScriptError::InternalError(rquickjs::Error::Unknown)
            }
            WorkerError::InvalidInputEncoding { .. } => {
                ScriptError::InvalidInputEncoding(rquickjs::Error::Unknown)
            }
            WorkerError::InvalidOutputDecoding { .. } => {
                ScriptError::InvalidOutputDecoding(rquickjs::Error::Unknown)
            }
            WorkerError::NoOutput => ScriptError::NoOutput,
            WorkerError::ExecutionException { message, stack } => {
                ScriptError::ExecutionException { message, stack }
            }
            WorkerError::InvalidPromise => ScriptError::InvalidPromise,
            WorkerError::ProcessTimeout => ScriptError::ProcessTimeout,
            WorkerError::OutOfMemory => ScriptError::OutOfMemory,
        }
    }
}

fn encode<T: Serialize>(msg: &T) -> anyhow::Result<Bytes> {
    Ok(rmp_serde::to_vec_named(msg)?.into())
}

fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> anyhow::Result<T> {
    Ok(rmp_serde::from_slice(bytes)?)
}

/// This handle manages an executed subprocess that can handle multiple concurrent transformations.
struct WorkerProcess {
    /// Send requests to the dedicated writer task.
    request_tx: mpsc::UnboundedSender<WorkerRequestMsg>,
    /// Oneshot senders waiting for their response, keyed by request ID.
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<WorkerResponse>>>>,
    next_id: AtomicU64,
    /// Keep the child alive for kill-on-drop.
    _child: Mutex<Child>,
}

impl WorkerProcess {
    fn spawn(exe_path: &PathBuf) -> anyhow::Result<Arc<Self>> {
        let mut child = Command::new(exe_path)
            .arg("transform-worker")
            .env_clear()
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = FramedWrite::new(
            child.stdin.take().expect("stdin piped"),
            LengthDelimitedCodec::new(),
        );
        let stdout = FramedRead::new(
            child.stdout.take().expect("stdout piped"),
            LengthDelimitedCodec::new(),
        );

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<WorkerResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let (request_tx, request_rx) = mpsc::unbounded_channel();

        tokio::spawn(Self::writer_task(stdin, request_rx));
        tokio::spawn(Self::reader_task(stdout, Arc::clone(&pending)));

        Ok(Arc::new(WorkerProcess {
            request_tx,
            pending,
            next_id: AtomicU64::new(0),
            _child: Mutex::new(child),
        }))
    }

    fn is_alive(&self) -> bool {
        !self.request_tx.is_closed()
    }

    async fn send(&self, req: WorkerRequestMsg) -> WorkerResponse {
        let id = req.id;
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        if self.request_tx.send(req).is_err() {
            self.pending.lock().await.remove(&id);
            return WorkerResponse::Err(WorkerError::InternalError {
                message: "worker subprocess exited".into(),
            });
        }

        match rx.await {
            Ok(response) => response,
            Err(_) => WorkerResponse::Err(WorkerError::InternalError {
                message: "worker subprocess exited".into(),
            }),
        }
    }

    async fn writer_task(
        mut stdin: FramedWrite<ChildStdin, LengthDelimitedCodec>,
        mut rx: mpsc::UnboundedReceiver<WorkerRequestMsg>,
    ) {
        while let Some(msg) = rx.recv().await {
            let Ok(bytes) = encode(&msg) else { break };
            if stdin.send(bytes).await.is_err() {
                break;
            }
        }
    }

    async fn reader_task(
        mut stdout: FramedRead<ChildStdout, LengthDelimitedCodec>,
        pending: Arc<Mutex<HashMap<u64, oneshot::Sender<WorkerResponse>>>>,
    ) {
        while let Some(frame) = stdout.next().await {
            match frame {
                Err(_) => break,
                Ok(bytes) => {
                    let Ok(msg) = decode::<WorkerResponseMsg>(&bytes) else {
                        continue;
                    };
                    let mut map = pending.lock().await;
                    if let Some(tx) = map.remove(&msg.id) {
                        let _ = tx.send(msg.response);
                    }
                }
            }
        }
        // Stream ended (subprocess exited); fail all in-flight requests.
        let mut map = pending.lock().await;
        for (_, tx) in map.drain() {
            let _ = tx.send(WorkerResponse::Err(WorkerError::InternalError {
                message: "worker subprocess exited unexpectedly".into(),
            }));
        }
    }
}

struct Worker {
    permits: Arc<Semaphore>,
    /// The single subprocess handle. Replaced when the process dies.
    process: Mutex<Option<Arc<WorkerProcess>>>,
    exe_path: PathBuf,
}

impl Worker {
    fn new(max_workers: usize, exe_path: PathBuf) -> Self {
        Worker {
            permits: Arc::new(Semaphore::new(max_workers)),
            process: Mutex::new(None),
            exe_path,
        }
    }

    async fn run_script(
        &self,
        script: String,
        input: String,
        max_duration: Duration,
    ) -> Result<String, ScriptError> {
        let _permit = self
            .permits
            .acquire()
            .await
            .expect("semaphore never closed");

        let handle = self.get_or_spawn().await?;

        let id = handle.next_id.fetch_add(1, Ordering::Relaxed);
        let req = WorkerRequestMsg {
            id,
            script,
            input,
            timeout_ms: max_duration.as_millis() as u64,
        };

        let deadline = max_duration + Duration::from_secs(1);
        match timeout(deadline, handle.send(req)).await {
            Ok(WorkerResponse::Ok(output)) => Ok(output),
            Ok(WorkerResponse::Err(e)) => Err(ScriptError::from(e)),
            Err(_elapsed) => {
                tracing::warn!("worker subprocess timed out, replacing it");
                *self.process.lock().await = None;
                Err(ScriptError::ProcessTimeout)
            }
        }
    }

    async fn get_or_spawn(&self) -> Result<Arc<WorkerProcess>, ScriptError> {
        let mut guard = self.process.lock().await;
        if let Some(handle) = guard.as_ref()
            && handle.is_alive()
        {
            return Ok(Arc::clone(handle));
        }
        let handle = WorkerProcess::spawn(&self.exe_path).map_err(|e| {
            tracing::error!(err = ?e, "failed to spawn transform worker");
            ScriptError::InternalError(rquickjs::Error::Unknown)
        })?;
        *guard = Some(Arc::clone(&handle));
        Ok(handle)
    }
}

// ---------------------------------------------------------------------------
// Global singleton
// ---------------------------------------------------------------------------

static WORKER: OnceLock<Worker> = OnceLock::new();

fn get_worker() -> &'static Worker {
    WORKER.get_or_init(|| {
        let max_workers = available_parallelism().map(|n| n.get()).unwrap_or(4);
        let exe_path = std::env::current_exe().expect("cannot determine current exe path");
        Worker::new(max_workers, exe_path)
    })
}

/// This function lets you run a specific script
pub async fn run_script(
    script: impl Into<String>,
    input: impl Into<String>,
    max_duration: Duration,
) -> Result<String, ScriptError> {
    get_worker()
        .run_script(script.into(), input.into(), max_duration)
        .await
}

#[cfg(target_os = "linux")]
fn apply_sandboxing() -> anyhow::Result<()> {
    use landlock::{
        ABI, Access, AccessFs, AccessNet, CompatLevel, Compatible, RestrictionStatus, Ruleset,
        RulesetAttr, RulesetStatus,
    };
    let abi = ABI::V4;
    let status: RestrictionStatus = Ruleset::default()
        .set_compatibility(CompatLevel::BestEffort)
        .handle_access(AccessFs::from_all(abi))?
        .handle_access(AccessNet::from_all(abi))?
        .create()?
        .restrict_self()?;
    if status.ruleset != RulesetStatus::FullyEnforced {
        tracing::warn!(
            ?status,
            "Landlock sandbox only partially enforced; some restrictions may be unsupported on this kernel"
        );
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn apply_sandboxing() -> anyhow::Result<()> {
    Ok(())
}

/// Worker event loop - this is what's run in the worker subprocess.
pub async fn run_as_worker() -> anyhow::Result<()> {
    apply_sandboxing()?;

    let stdout = Arc::new(Mutex::new(FramedWrite::new(
        tokio::io::stdout(),
        LengthDelimitedCodec::new(),
    )));
    let mut reader = FramedRead::new(tokio::io::stdin(), LengthDelimitedCodec::new());
    while let Some(frame) = reader.next().await {
        let bytes = frame?;
        let req: WorkerRequestMsg = rmp_serde::from_slice(&bytes)?;

        let stdout = Arc::clone(&stdout);
        tokio::spawn(async move {
            let duration = Duration::from_millis(req.timeout_ms);
            let result = crate::engine::run_script(req.script, req.input, duration).await;
            let response = match result {
                Ok(output) => WorkerResponse::Ok(output),
                Err(e) => WorkerResponse::Err(WorkerError::from(e)),
            };
            let msg = WorkerResponseMsg {
                id: req.id,
                response,
            };
            if let Ok(bytes) = encode(&msg) {
                let mut w = stdout.lock().await;
                let _ = w.send(bytes).await;
            }
        });
    }

    Ok(())
}
