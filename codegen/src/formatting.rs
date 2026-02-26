use std::{fmt::Write as _, io, path::PathBuf, process::ExitCode};

use async_process::{Command, Stdio};
use futures_lite::future;

pub(crate) fn run() -> ExitCode {
    let (r1, (r2, r3)) = async_io::block_on(future::zip(
        format_rust_clients(),
        future::zip(format_go_client(), format_python_client()),
    ));
    let mut exit_code = ExitCode::SUCCESS;
    for result in [r1, r2, r3] {
        if let Err(e) = result {
            eprintln!("{e}\n");
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}

async fn format_rust_clients() -> io::Result<()> {
    exec(
        "cargo",
        [
            "+nightly",
            "fmt",
            "--package=diom-client",
            "--package=diom-cli",
        ],
    )
    .await
}

async fn format_go_client() -> io::Result<()> {
    ContainerizedFormatter {
        container: "goimports",
        mounts: &[("clients/go", "/go/diom")],
        cmd: &["goimports", "-w", "/go/diom"],
    }
    .run()
    .await
}

async fn format_python_client() -> io::Result<()> {
    ContainerizedFormatter {
        container: "ruff",
        mounts: &[("clients/python", "/clients/python")],
        cmd: &[
            "sh",
            "-c",
            "ruff check --fix /clients/python/diom && ruff format /clients/python/diom",
        ],
    }
    .run()
    .await
}

struct ContainerizedFormatter<'a> {
    /// Container name.
    ///
    /// A file named `{container}.Containerfile` must exist in codegen/formatters.
    /// An image named `diom-formatter-{container}` will be built from it.
    container: &'a str,

    /// What directories to mount into the container.
    ///
    /// A list of (source, destination) tuples.
    /// Source paths are relative to the repository root.
    mounts: &'a [(&'a str, &'a str)],

    /// The command (first item) and its arguments (remaining items) to run inside the container.
    cmd: &'a [&'a str],
}

impl ContainerizedFormatter<'_> {
    async fn run(&self) -> io::Result<()> {
        let Self {
            container,
            mounts,
            cmd,
        } = self;

        let tag = format!("diom-formatter-{container}");
        let containerfile_path = format!("codegen/formatters/{container}.Containerfile");
        let mounts: Vec<_> = mounts
            .iter()
            .map(|(src, dst)| {
                // docker requires that all bind mount paths be absolute
                let path = PathBuf::from(src);
                let src = path.canonicalize().unwrap();
                let src = src.to_string_lossy();
                format!("--mount=type=bind,src={src},dst={dst}")
            })
            .collect();

        let base = if which::which("podman").is_ok() {
            "podman"
        } else if which::which("docker").is_ok() {
            "docker"
        } else {
            return Err(io::Error::other("could not find podman or docker in $PATH"));
        };

        let ctx_dir = "codegen/formatters";
        let args = vec!["build", "-t", &tag, "-f", &containerfile_path, ctx_dir];
        exec(base, args).await?;
        let args = ["run"]
            .into_iter()
            .chain(mounts.iter().map(|m| m.as_str()))
            .chain([tag.as_str()])
            .chain(cmd.iter().copied());
        exec(base, args).await?;

        Ok(())
    }
}

async fn exec(cmd: &str, args: impl IntoIterator<Item = &str>) -> io::Result<()> {
    let args = args.into_iter().collect::<Vec<_>>();
    tracing::debug!(cmd, ?args, "running formatter command");
    let output = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| {
            let kind = e.kind();
            let msg = format!("failed to run {cmd}: e");
            io::Error::new(kind, msg)
        })?;

    if !output.status.success() {
        let mut msg = format!("{cmd} failed with status {}\n", output.status);
        add_cmd_output(&mut msg, "stdout", &output.stdout);
        add_cmd_output(&mut msg, "stderr", &output.stderr);
        return Err(io::Error::other(msg));
    }

    Ok(())
}

fn add_cmd_output(msg: &mut String, arg: &str, output: &[u8]) {
    if output.is_empty() {
        return;
    }

    writeln!(msg, "-- {arg} --").unwrap();

    let output = String::from_utf8_lossy(output);
    for line in output.lines() {
        writeln!(msg, "| {line}").unwrap();
    }
}
