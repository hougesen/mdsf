#[cfg(test)]
mod test_cli {
    use std::io::Write;

    use tempfile::tempdir;

    const BROKEN_CODE: &'static str = "```rust
fn add(a:i32,b:i32)->i32{a+b}
```
";

    const FORMATTED_CODE: &'static str = "```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
";

    fn mdsf_command(path: &std::path::Path) -> assert_cmd::Command {
        let mut cmd = assert_cmd::Command::cargo_bin("mdsf").unwrap();

        cmd.current_dir(path);

        cmd
    }

    fn setup_test_input(dir: &std::path::Path, code: &str) -> tempfile::NamedTempFile {
        let mut b = tempfile::Builder::new();

        b.rand_bytes(12).suffix(".md");

        let mut f = b.tempfile_in(dir).unwrap();

        f.write_all(code.as_bytes()).unwrap();
        f.flush().unwrap();

        f
    }

    #[test]
    fn validate_help_command() {
        let dir = tempdir().unwrap();

        let cmd = mdsf_command(dir.path()).arg("--help").assert().success();

        let output = cmd.get_output();

        assert!(!output.stdout.is_empty());
    }

    #[test]
    fn validate_completions_command() {
        let dir = tempdir().unwrap();

        let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

        for shell in shells {
            let cmd = mdsf_command(dir.path())
                .arg("completions")
                .arg(shell)
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }
    }

    #[test]
    fn validate_cache_prune_command() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path())
            .arg("cache-prune")
            .assert()
            .success();
    }

    #[test]
    fn validate_check_command_with_formatted_input() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path()).arg("init").assert().success();

        let file = setup_test_input(dir.path(), FORMATTED_CODE);

        mdsf_command(dir.path())
            .arg("verify")
            .arg(file.path())
            .assert()
            .success();
    }

    #[test]
    fn validate_check_command_with_formatted_input_stdin() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path()).arg("init").assert().success();

        mdsf_command(dir.path())
            .arg("verify")
            .arg("--stdin")
            .write_stdin(FORMATTED_CODE)
            .assert()
            .success();
    }

    #[test]
    fn validate_check_command_with_broken_input() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path()).arg("init").assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        mdsf_command(dir.path())
            .arg("verify")
            .arg(file.path())
            .assert()
            .failure();
    }

    #[test]
    fn validate_check_command_with_broken_input_stdin() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path()).arg("init").assert().success();

        mdsf_command(dir.path())
            .arg("verify")
            .arg("--stdin")
            .write_stdin(BROKEN_CODE)
            .assert()
            .failure();
    }

    #[test]
    fn validate_check_command_fails_without_input() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path()).arg("verify").assert().failure();
    }
}
