mod commands;

fn main() {
    if let Err(error) = commands::run_command() {
        mdsf::error::exit_with_error(&error)
    }

    if mdsf::error::HAS_ERROR.load(core::sync::atomic::Ordering::Relaxed) {
        std::process::exit(1)
    }
}
