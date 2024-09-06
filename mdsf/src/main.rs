use mdsf::terminal::print_error;

mod commands;

fn main() {
    if let Err(error) = commands::execute_command() {
        print_error(&error);

        std::process::exit(1);
    }
}
