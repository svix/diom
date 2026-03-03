use std::{fmt::Write as _, io};

use async_process::{Command, Stdio};
use fs_err as fs;

pub(crate) struct OutputDirectory {
    pub path: &'static str,
    pub templates: &'static [&'static str],
    /// Whether the directory is managed by codegen.
    ///
    /// If this is `true`, the directory will be emptied before generating code.
    pub managed: bool,
}

impl OutputDirectory {
    pub(crate) const fn managed_dir(
        path: &'static str,
        templates: &'static [&'static str],
    ) -> Self {
        Self {
            path,
            templates,
            managed: true,
        }
    }

    pub(crate) const fn unmanaged_dir(
        path: &'static str,
        templates: &'static [&'static str],
    ) -> Self {
        Self {
            path,
            templates,
            managed: false,
        }
    }
}

pub(crate) struct ContainerizedFormatter<'a> {
    /// Container name.
    ///
    /// A file named `{container}.Containerfile` must exist in codegen/formatters.
    /// An image named `diom-formatter-{container}` will be built from it.
    pub container: &'a str,

    /// What directories to mount into the container.
    ///
    /// A list of (source, destination) tuples.
    /// Source paths are relative to the repository root.
    pub mounts: &'a [(&'a str, &'a str)],

    /// The command (first item) and its arguments (remaining items) to run inside the container.
    pub cmd: &'a [&'a str],
}

impl ContainerizedFormatter<'_> {
    pub(crate) async fn run(&self) -> io::Result<()> {
        let Self {
            container,
            mounts,
            cmd,
        } = self;

        let tag = format!("diom-formatter-{container}");
        let containerfile_path = format!("codegen/formatters/{container}.Containerfile");
        let mounts = mounts
            .iter()
            .map(|(src, dst)| {
                // docker requires that all bind mount paths be absolute
                let src = fs::canonicalize(src)?;
                let src = src.display();
                Ok(format!("--mount=type=bind,src={src},dst={dst}"))
            })
            .collect::<io::Result<Vec<_>>>()?;

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

pub(crate) async fn exec(cmd: &str, args: impl IntoIterator<Item = &str>) -> io::Result<()> {
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
