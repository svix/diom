use std::process::ExitCode;

use futures_lite::future;

use crate::{go::format_go_client, python::format_python_client, rust::format_rust_clients};

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
