#[allow(dead_code)]
mod cli_command;

mod completions {
    use predicates::prelude::PredicateBooleanExt;

    use crate::cli_command::completions_command;

    #[test]
    fn help_arg_outputs_message() {
        completions_command()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn outputs_shell_completions() {
        let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

        for shell in shells {
            completions_command()
                .arg(shell)
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }
    }
}
