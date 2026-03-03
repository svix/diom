use std::{io, process::ExitCode};

use futures_lite::future;

use crate::utils::{ContainerizedFormatter, exec};

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
            "--package=coyote-client",
            "--package=coyote-cli",
        ],
    )
    .await?;
    Ok(())
}

async fn format_go_client() -> io::Result<()> {
    ContainerizedFormatter {
        container: "goimports",
        mounts: &[("clients/go", "/go/coyote")],
        cmd: &["goimports", "-w", "/go/coyote"],
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
            "ruff check --fix /clients/python/coyote && ruff format /clients/python/coyote",
        ],
    }
    .run()
    .await
}
