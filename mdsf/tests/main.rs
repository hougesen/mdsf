use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

fn mdsf_command() -> Result<std::process::Command, assert_cmd::cargo::CargoError> {
    std::process::Command::cargo_bin("mdsf")
}

#[test]
fn validate_help_command() -> Result<(), assert_cmd::cargo::CargoError> {
    let cmd = mdsf_command()?.arg("--help").assert().success();

    let output = cmd.get_output();

    assert!(!output.stdout.is_empty());

    Ok(())
}

#[test]
fn validate_completions_command() -> Result<(), assert_cmd::cargo::CargoError> {
    let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

    for shell in shells {
        let cmd = mdsf_command()?
            .arg("completions")
            .arg(shell)
            .assert()
            .success();

        let output = cmd.get_output();

        assert!(!output.stdout.is_empty());
    }

    Ok(())
}

#[test]
fn validate_cache_prune_command() -> Result<(), assert_cmd::cargo::CargoError> {
    mdsf_command()?.arg("cache-prune").assert().success();

    Ok(())
}
