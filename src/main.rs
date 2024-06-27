use mdsf::terminal::print_error;

mod commands;

#[tokio::main]
async fn main() {
    if let Err(error) = commands::execute_command().await {
        print_error(&error);

        std::process::exit(1);
    }
}
