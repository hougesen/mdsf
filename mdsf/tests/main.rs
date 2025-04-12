#[cfg(test)]
mod test_cli {
    use std::io::Write;

    use predicates::prelude::PredicateBooleanExt;
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

        mdsf_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn version_arg_outputs_message() {
        let dir = tempdir().unwrap();

        mdsf_command(dir.path())
            .arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    mod help {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn command_outputs_help_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }
    }

    mod completions {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("completions")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn outputs_shell_completions() {
            let dir = tempdir().unwrap();

            let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

            for shell in shells {
                mdsf_command(dir.path())
                    .arg("completions")
                    .arg(shell)
                    .assert()
                    .success()
                    .stdout(predicates::str::is_empty().not());
            }
        }
    }

    mod init {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("init")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
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

        #[test]
        fn supports_log_level_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("init")
                .arg("--log-level")
                .arg("off")
                .assert()
                .success()
                .stderr(predicates::str::is_empty());
        }
    }

    mod cache_prune {
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use super::mdsf_command;

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("cache-prune")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }

        #[test]
        fn removes_existing_caches() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("cache-prune")
                .assert()
                .success();
        }

        #[test]
        fn supports_log_level_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("cache-prune")
                .arg("--log-level")
                .arg("off")
                .assert()
                .success()
                .stderr(predicates::str::is_empty());
        }
    }

    mod verify {
        use mdsf::{cli::OnMissingLanguageDefinition, config::MdsfConfig};
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use super::{BROKEN_CODE, FORMATTED_CODE, mdsf_command, setup_test_input};

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
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

        #[test]
        fn supports_log_level_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("init").assert().success();

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--stdin")
                .arg("--log-level")
                .arg("off")
                .write_stdin(FORMATTED_CODE)
                .assert()
                .success()
                .stderr(predicates::str::is_empty());
        }

        #[test]
        fn on_missing_language_definition_prioritize_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_ignore_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_ignore_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Ignore),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("fail")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Fail),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_fast_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("fail-fast")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_fast_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("verify")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }
    }

    mod format {
        use mdsf::{cli::OnMissingLanguageDefinition, config::MdsfConfig};
        use predicates::prelude::PredicateBooleanExt;
        use tempfile::tempdir;

        use crate::test_cli::{FORMATTED_CODE, mdsf_command};

        use super::{BROKEN_CODE, setup_test_input};

        #[test]
        fn help_arg_outputs_message() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("format")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
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

            mdsf_command(dir.path())
                .arg("format")
                .arg("--stdin")
                .write_stdin(BROKEN_CODE)
                .assert()
                .success()
                .stdout(predicates::str::contains(FORMATTED_CODE));
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
                let mut cmd = mdsf_command(dir.path())
                    .arg("format")
                    .arg("--cache")
                    .arg(file.path())
                    .assert()
                    .success();

                let output = std::fs::read_to_string(file.path()).unwrap();
                assert_eq!(output, FORMATTED_CODE);

                if i > 0 {
                    cmd = cmd.stderr(predicates::str::contains("(unchanged)"));
                }

                if i > 1 {
                    cmd.stderr(predicates::str::contains("(cached)"));
                }
            }
        }

        #[test]
        fn fails_without_input() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path()).arg("format").assert().failure();
        }

        #[test]
        fn supports_log_level_argument() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("format")
                .arg("--log-level")
                .arg("off")
                .assert()
                .failure()
                .stderr(predicates::str::is_empty());
        }

        #[test]
        fn supports_config_path_argument() {
            let dir = tempdir().unwrap();

            let dir2 = tempdir().unwrap();
            mdsf_command(dir2.path()).arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--config")
                .arg(dir2.path().join("mdsf.json"))
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path()).unwrap();
            assert_eq!(output, FORMATTED_CODE);
        }

        #[test]
        fn it_should_error_if_config_is_not_valid_json() {
            let dir = tempdir().unwrap();

            std::fs::write(dir.path().join("mdsf.json"), "{ this is not valid json }").unwrap();

            mdsf_command(dir.path())
                .arg("format")
                .arg("--config")
                .arg("thisdoesnotexist.json")
                .arg(".")
                .assert()
                .failure();
        }

        #[test]
        fn it_should_error_if_config_path_does_not_exist() {
            let dir = tempdir().unwrap();

            mdsf_command(dir.path())
                .arg("format")
                .arg("--config")
                .arg("thisdoesnotexist.json")
                .arg(".")
                .assert()
                .failure();
        }

        #[test]
        fn on_missing_language_definition_prioritize_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_ignore_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_ignore_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Ignore),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("fail")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Fail),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_fast_cli() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("fail-fast")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }

        #[test]
        fn on_missing_language_definition_fail_fast_config() {
            let dir = tempdir().unwrap();

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config).unwrap(),
            )
            .unwrap();

            let file = setup_test_input(dir.path(), BROKEN_CODE);

            mdsf_command(dir.path())
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));
        }
    }
}
