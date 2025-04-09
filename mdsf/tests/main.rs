#[cfg(test)]
mod test_cli {
    use std::io::Write;

    use tempfile::tempdir;

    const BROKEN_CODE: &str = "```rust
fn add(a:i32,b:i32)->i32{a+b}
```
";

    const FORMATTED_CODE: &str = "```rust
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
        fn fails_if_config_exists() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            mdsf_command(dir.path()).arg("init").assert().failure();
        }

        #[test]
        fn force_config_arg() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            mdsf_command(dir.path()).arg("init").assert().failure();

            mdsf_command(dir.path())
                .arg("init")
                .arg("--force")
                .assert()
                .success();
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
        fn success_with_formatted_input_and_debug_arg() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let file = setup_test_input(dir.path(), FORMATTED_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--debug")
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
        fn accepts_multiple_file_paths_formatted() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("verify");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(FORMATTED_CODE, output);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_formatted_with_threads_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("verify").arg("--threads").arg("4");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(FORMATTED_CODE, output);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_formatted_with_threads_argument_zero() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("verify").arg("--threads").arg("0");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(FORMATTED_CODE, output);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_broken() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("verify");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().failure();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(BROKEN_CODE, output);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_mixed() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("verify");

            let mut files = Vec::new();

            for i in 0..5 {
                let input = if i % 2 == 0 {
                    FORMATTED_CODE
                } else {
                    BROKEN_CODE
                };

                let file = setup_test_input(dir.path(), input);

                cmd.arg(file.path());

                files.push((file, input));
            }

            cmd.assert().failure();

            // Validate the files wasn't changed
            for (file, input) in files {
                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(input, output);
            }
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
        fn formats_broken_input_with_debug_arg() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--debug")
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
        fn accepts_multiple_file_paths() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("format");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, FORMATTED_CODE);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_with_thread_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("format").arg("--threads").arg("4");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, FORMATTED_CODE);
            }
        }

        #[test]
        fn accepts_multiple_file_paths_with_thread_argument_zero() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path());

            cmd.arg("format").arg("--threads").arg("0");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE);

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path()).unwrap();

                assert_eq!(output, FORMATTED_CODE);
            }
        }

        #[test]
        fn format_with_cache_arg() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            for i in 0..10 {
                let cmd = mdsf_command(dir.path())
                    .arg("format")
                    .arg("--cache")
                    .arg(file.path())
                    .assert()
                    .success();

                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(output, FORMATTED_CODE);

                let stderr = String::from_utf8(cmd.get_output().stderr.clone()).unwrap();
                if i > 0 {
                    assert!(stderr.contains("(unchanged)"));
                }

                if i > 1 {
                    assert!(stderr.contains("(cached)"));
                }
            }
        }

        #[test]
        fn fails_without_input() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("verify").assert().failure();
        }
    }
}
