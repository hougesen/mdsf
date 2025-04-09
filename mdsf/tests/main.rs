use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn validate_help_command() -> Result<(), assert_cmd::cargo::CargoError> {
    let cmd = std::process::Command::cargo_bin("mdsf")?
        .arg("--help")
        .assert()
        .success();

    let output = cmd.get_output();

    assert!(!output.stdout.is_empty());

    Ok(())
}

#[test]
fn validate_completions_command() -> Result<(), assert_cmd::cargo::CargoError> {
    let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

    for shell in shells {
        let cmd = std::process::Command::cargo_bin("mdsf")?
            .arg("completions")
            .arg(shell)
            .assert()
            .success();

        let output = cmd.get_output();

        assert!(!output.stdout.is_empty());
    }

    Ok(())
}
