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
    fn help_arg_outputs_message() {
        let dir = tempdir().unwrap();

        let cmd = mdsf_command(dir.path()).arg("--help").assert().success();

        let output = cmd.get_output();

        assert!(!output.stdout.is_empty());
    }

    #[test]
    fn version_arg_outputs_message() {
        let dir = tempdir().unwrap();

        let cmd = mdsf_command(dir.path()).arg("--version").assert().success();

        let output = cmd.get_output();

        assert!(!output.stdout.is_empty());
    }

    mod help {
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn command_outputs_help_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path()).arg("help").assert().success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }
    }

    mod completions {
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path())
                .arg("completions")
                .arg("--help")
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }

        #[test]
        fn outputs_shell_completions() {
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
    }

    mod init {
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path())
                .arg("init")
                .arg("--help")
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }

        #[test]
        fn creates_a_config_file() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let config_file_created = dir.path().join("mdsf.json").try_exists().unwrap();

            assert!(config_file_created);
        }

        #[test]
        #[ignore]
        fn fails_if_config_exists() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            mdsf_command(dir.path()).arg("init").assert().failure();
        }
    }

    mod cache_prune {
        use tempfile::tempdir;

        use super::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path())
                .arg("cache-prune")
                .arg("--help")
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }

        #[test]
        fn removes_existing_caches() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("cache-prune")
                .assert()
                .success();
        }
    }

    mod verify {
        use tempfile::tempdir;

        use super::{BROKEN_CODE, FORMATTED_CODE, mdsf_command, setup_test_input};

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path())
                .arg("verify")
                .arg("--help")
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }

        #[test]
        fn success_with_formatted_input() {
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
        fn success_with_formatted_input_stdin() {
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
        fn fails_with_broken_input() {
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
        fn fails_with_broken_input_stdin() {
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
        #[ignore]
        fn accepts_multiple_file_paths() {
            todo!()
        }

        #[test]
        fn fails_without_input() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("verify").assert().failure();
        }
    }

    mod format {
        use tempfile::tempdir;

        use crate::test_cli::{FORMATTED_CODE, mdsf_command};

        use super::{BROKEN_CODE, setup_test_input};

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            let cmd = mdsf_command(dir.path())
                .arg("verify")
                .arg("--help")
                .assert()
                .success();

            let output = cmd.get_output();

            assert!(!output.stdout.is_empty());
        }

        #[test]
        fn formats_broken_input() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path()).unwrap();

            assert_eq!(output, FORMATTED_CODE);
        }

        #[test]
        fn formats_broken_input_stdin() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let cmd = mdsf_command(dir.path())
                .arg("format")
                .arg("--stdin")
                .write_stdin(BROKEN_CODE)
                .assert()
                .success();

            let stdout_output = String::from_utf8(cmd.get_output().stdout.clone()).unwrap();

            assert_eq!(stdout_output.trim_end(), FORMATTED_CODE.trim_end());
        }

        #[test]
        #[ignore]
        fn accepts_multiple_file_paths() {
            todo!()
        }

        #[test]
        fn fails_without_input() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("verify").assert().failure();
        }
    }
}
