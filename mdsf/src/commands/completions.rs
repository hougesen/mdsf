use clap::CommandFactory;
use mdsf::cli::{self, Cli, CompletionsCommandArguments};

#[inline]
pub fn run(args: &CompletionsCommandArguments, buffer: &mut impl std::io::Write) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    match args.shell {
        cli::Shell::Bash => {
            clap_complete::generate(clap_complete::Shell::Bash, &mut cmd, cmd_name, buffer);
        }
        cli::Shell::Elvish => {
            clap_complete::generate(clap_complete::Shell::Elvish, &mut cmd, cmd_name, buffer);
        }
        cli::Shell::PowerShell => {
            clap_complete::generate(clap_complete::Shell::PowerShell, &mut cmd, cmd_name, buffer);
        }
        cli::Shell::Fish => {
            clap_complete::generate(clap_complete::Shell::Fish, &mut cmd, cmd_name, buffer);
        }
        cli::Shell::Zsh => {
            clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd, cmd_name, buffer);
        }
        cli::Shell::Nushell => {
            clap_complete::generate(clap_complete_nushell::Nushell, &mut cmd, cmd_name, buffer);
        }
    }
}

#[cfg(test)]
mod test_run {
    use mdsf::cli::CompletionsCommandArguments;

    #[test]
    fn it_should_write_bash_completions() {
        let shells = [
            mdsf::cli::Shell::Bash,
            mdsf::cli::Shell::Elvish,
            mdsf::cli::Shell::PowerShell,
            mdsf::cli::Shell::Fish,
            mdsf::cli::Shell::Zsh,
            mdsf::cli::Shell::Nushell,
        ];

        for shell in shells {
            let args = CompletionsCommandArguments { shell };

            let mut buffer = Vec::new();

            super::run(&args, &mut buffer);

            assert!(!buffer.is_empty());
        }
    }
}
