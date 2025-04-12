use mdsf::error::{HAS_ERROR, exit_with_error};

mod commands;

fn main() {
    if let Err(error) = commands::run_command() {
        exit_with_error(error);
    }

    if HAS_ERROR.load(core::sync::atomic::Ordering::Relaxed) {
        std::process::exit(1);
    }
}
