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
    apply_seccomp()?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn apply_sandboxing() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn apply_seccomp() -> anyhow::Result<()> {
    use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, TargetArch};
    use std::collections::BTreeMap;

    let arch = match std::env::consts::ARCH {
        "x86_64" => TargetArch::x86_64,
        "aarch64" => TargetArch::aarch64,
        other => {
            tracing::warn!(
                arch = other,
                "seccomp BPF not supported on this architecture, skipping"
            );
            return Ok(());
        }
    };

    // Allowlist: only the syscalls listed here are permitted inside the worker
    // subprocess. Everything else (socket, open/openat, execve, fork, …) causes
    // the kernel to send SIGSYS and kill the process immediately.
    //
    // Notably absent (blocked):
    //   file access  – open, openat, creat, unlink, rename, …
    //   networking   – socket, connect, bind, listen, sendto, recvfrom, …
    //   new processes – fork, vfork, execve, execveat
    //   (clone is allowed; it is needed for tokio/rquickjs thread pools)
    #[rustfmt::skip]
    let allowed: &[libc::c_long] = &[
        // I/O on already-open file descriptors (stdin / stdout are pre-opened)
        libc::SYS_read,    libc::SYS_write,
        libc::SYS_readv,   libc::SYS_writev,
        libc::SYS_pread64, libc::SYS_pwrite64,
        libc::SYS_close,
        libc::SYS_fstat,   libc::SYS_newfstatat,
        libc::SYS_fcntl,   // non-blocking / close-on-exec flags
        libc::SYS_ioctl,   // terminal/pipe detection by libc/tracing

        // Memory management
        libc::SYS_mmap,    libc::SYS_mprotect, libc::SYS_munmap,
        libc::SYS_brk,     libc::SYS_mremap,   libc::SYS_madvise,

        // Threads and synchronisation (clone = thread creation; no fork/exec)
        libc::SYS_clone,
        libc::SYS_clone3,
        libc::SYS_futex,
        libc::SYS_set_robust_list, libc::SYS_get_robust_list,
        libc::SYS_set_tid_address,
        libc::SYS_sched_yield, libc::SYS_sched_getaffinity,

        // Signals
        libc::SYS_rt_sigaction, libc::SYS_rt_sigprocmask,
        libc::SYS_rt_sigreturn, libc::SYS_sigaltstack,

        // Time
        libc::SYS_nanosleep,     libc::SYS_clock_gettime,
        libc::SYS_clock_getres,  libc::SYS_clock_nanosleep,
        libc::SYS_gettimeofday,
        libc::SYS_timerfd_settime, libc::SYS_timerfd_gettime,
        libc::SYS_timerfd_create,

        // Async I/O multiplexing (tokio epoll reactor)
        libc::SYS_epoll_create1, libc::SYS_epoll_ctl,
        libc::SYS_epoll_wait,    libc::SYS_epoll_pwait,
        libc::SYS_epoll_pwait2,
        libc::SYS_eventfd2,
        libc::SYS_poll,          libc::SYS_ppoll,
        libc::SYS_select,        libc::SYS_pselect6,
        libc::SYS_pipe2,         // waker pipe used by some tokio internals

        // Process / thread identity (read-only)
        libc::SYS_getpid, libc::SYS_gettid,
        libc::SYS_getuid, libc::SYS_geteuid,
        libc::SYS_getgid, libc::SYS_getegid,
        libc::SYS_prlimit64,

        // Miscellaneous (allocator / runtime init)
        libc::SYS_prctl,     // thread naming, PR_SET_NAME, etc.
        libc::SYS_arch_prctl, // x86-64 TLS segment setup
        libc::SYS_getrandom, // entropy for hash maps, UUIDs
        libc::SYS_rseq,      // glibc 2.35+ restartable sequences

        // Exit
        libc::SYS_exit, libc::SYS_exit_group,
        libc::SYS_restart_syscall,

        // Extra needed by tokio
        libc::SYS_openat
    ];

    let rules: BTreeMap<i64, Vec<seccompiler::SeccompRule>> =
        allowed.iter().map(|&nr| (nr, vec![])).collect();

    let filter = SeccompFilter::new(
        rules,
        SeccompAction::KillProcess,
        SeccompAction::Allow,
        arch,
    )?;

    let prog: BpfProgram = filter.try_into()?;
    if let Err(e) = seccompiler::apply_filter(&prog) {
        tracing::warn!(error = %e, "failed to apply seccomp BPF filter, worker will run unsandboxed");
        return Ok(());
    }

    tracing::debug!("seccomp BPF filter applied to worker subprocess");
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
