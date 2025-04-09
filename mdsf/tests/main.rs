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
