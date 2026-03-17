use std::{
    backtrace::{Backtrace, BacktraceStatus},
    panic::PanicHookInfo,
};

pub(crate) fn tracing_panic_hook(panic_info: &PanicHookInfo<'_>) {
    let backtrace = Backtrace::capture();
    let (backtrace, note) = match backtrace.status() {
        BacktraceStatus::Disabled => (
            None,
            Some("run with `RUST_BACKTRACE=1` environment variable to display a backtrace"),
        ),
        BacktraceStatus::Unsupported => {
            (None, Some("backtraces are not supported on this platform"))
        }
        BacktraceStatus::Captured => (Some(backtrace), None),
        _ => (None, Some("error capturing backtrace")),
    };

    let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        s
    } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
        s.as_str()
    } else {
        ""
    };

    tracing::error!(
        thread.name = std::thread::current().name(),
        thread.id = ?std::thread::current().id(),
        process.id = std::process::id(),
        panic.payload = payload,
        panic.location = panic_info.location().map(tracing::field::display),
        panic.backtrace = backtrace.map(tracing::field::display),
        panic.note = note,
        "{} panicked at",
        env!("CARGO_PKG_NAME")
    );
}

pub(crate) fn setup_tracing_panic_handler() {
    std::panic::set_hook(Box::new(tracing_panic_hook))
}
