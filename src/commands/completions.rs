use clap::CommandFactory;
use mdsf::cli::{Cli, CompletionsCommandArguments};

pub fn run(args: &CompletionsCommandArguments) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    clap_complete::generate(args.shell, &mut cmd, cmd_name, &mut std::io::stdout());
}
