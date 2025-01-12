use clap::CommandFactory;
use mdsf::cli::{self, Cli, CompletionsCommandArguments};

#[inline]
pub fn run(args: &CompletionsCommandArguments) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    match args.shell {
        cli::Shell::Bash => clap_complete::generate(
            clap_complete::Shell::Bash,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        cli::Shell::Elvish => clap_complete::generate(
            clap_complete::Shell::Elvish,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        cli::Shell::PowerShell => clap_complete::generate(
            clap_complete::Shell::PowerShell,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        cli::Shell::Fish => clap_complete::generate(
            clap_complete::Shell::Fish,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        cli::Shell::Zsh => clap_complete::generate(
            clap_complete::Shell::Zsh,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        cli::Shell::Nushell => clap_complete::generate(
            clap_complete_nushell::Nushell,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),
    };
}
