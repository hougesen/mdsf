use mdsf::error::exit_with_error;

mod commands;

fn main() {
    if let Err(error) = commands::run_command() {
        exit_with_error(error);
    }
}
