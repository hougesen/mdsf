#[cfg(test)]
mod test_cli {
    use std::io::Write;

    use predicates::prelude::PredicateBooleanExt;

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

    fn mdsf_command(
        path: &std::path::Path,
    ) -> Result<assert_cmd::Command, assert_cmd::cargo::CargoError> {
        let mut cmd = assert_cmd::Command::cargo_bin("mdsf")?;

        cmd.current_dir(path);

        Ok(cmd)
    }

    fn setup_test_dir() -> Result<tempfile::TempDir, std::io::Error> {
        tempfile::TempDir::with_prefix("mdsf")
    }

    fn setup_test_input(
        dir: &std::path::Path,
        code: &str,
    ) -> std::io::Result<tempfile::NamedTempFile> {
        let mut b = tempfile::Builder::new();

        b.rand_bytes(12).suffix(".md");

        let mut f = b.tempfile_in(dir)?;

        f.write_all(code.as_bytes())?;
        f.flush()?;

        Ok(f)
    }

    #[test]
    fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
        let dir = setup_test_dir()?;

        mdsf_command(dir.path())?
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());

        Ok(())
    }

    #[test]
    fn version_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
        let dir = setup_test_dir()?;

        mdsf_command(dir.path())?
            .arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());

        Ok(())
    }

    mod help {
        use predicates::prelude::PredicateBooleanExt;

        use super::setup_test_dir;
        use crate::test_cli::mdsf_command;

        #[test]
        fn command_outputs_help_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }
    }

    mod completions {
        use predicates::prelude::PredicateBooleanExt;

        use super::setup_test_dir;
        use crate::test_cli::mdsf_command;

        #[test]
        fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("completions")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }

        #[test]
        fn outputs_shell_completions() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

            for shell in shells {
                mdsf_command(dir.path())?
                    .arg("completions")
                    .arg(shell)
                    .assert()
                    .success()
                    .stdout(predicates::str::is_empty().not());
            }

            Ok(())
        }
    }

    mod init {
        use predicates::prelude::PredicateBooleanExt;

        use crate::test_cli::{mdsf_command, setup_test_dir};

        #[test]
        fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("init")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }

        #[test]
        fn creates_a_config_file() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let config_file_created = dir.path().join("mdsf.json").try_exists()?;

            assert!(config_file_created);

            Ok(())
        }

        #[test]
        fn fails_if_config_exists() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?.arg("init").assert().failure();

            Ok(())
        }

        #[test]
        fn force_config_arg() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?.arg("init").assert().failure();

            mdsf_command(dir.path())?
                .arg("init")
                .arg("--force")
                .assert()
                .success();

            Ok(())
        }

        #[test]
        fn supports_log_level_argument() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("init")
                .arg("--log-level")
                .arg("off")
                .assert()
                .success()
                .stderr(predicates::str::is_empty());

            Ok(())
        }
    }

    mod cache_prune {
        use predicates::prelude::PredicateBooleanExt;

        use super::{mdsf_command, setup_test_dir};

        #[test]
        fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("cache-prune")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }

        #[test]
        fn removes_existing_caches() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("cache-prune")
                .assert()
                .success();

            Ok(())
        }

        #[test]
        fn supports_log_level_argument() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("cache-prune")
                .arg("--log-level")
                .arg("off")
                .assert()
                .success()
                .stderr(predicates::str::is_empty());

            Ok(())
        }
    }

    mod verify {
        use mdsf::{cli::OnMissingLanguageDefinition, config::MdsfConfig};
        use predicates::prelude::PredicateBooleanExt;

        use super::{BROKEN_CODE, FORMATTED_CODE, mdsf_command, setup_test_dir, setup_test_input};

        #[test]
        fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }

        #[test]
        fn success_with_formatted_input() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), FORMATTED_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg(file.path())
                .assert()
                .success();

            Ok(())
        }

        #[test]
        fn success_with_formatted_input_and_debug_arg() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), FORMATTED_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--debug")
                .arg(file.path())
                .assert()
                .success();

            Ok(())
        }

        #[test]
        fn success_with_formatted_input_stdin() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--stdin")
                .write_stdin(FORMATTED_CODE)
                .assert()
                .success();

            Ok(())
        }

        #[test]
        fn fails_with_broken_input() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg(file.path())
                .assert()
                .failure();

            Ok(())
        }

        #[test]
        fn fails_with_broken_input_stdin() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--stdin")
                .write_stdin(BROKEN_CODE)
                .assert()
                .failure();

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_formatted() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("verify");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(FORMATTED_CODE, output);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_formatted_with_threads_argument()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("verify").arg("--threads").arg("4");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(FORMATTED_CODE, output);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_formatted_with_threads_argument_zero()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("verify").arg("--threads").arg("0");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), FORMATTED_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(FORMATTED_CODE, output);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_broken() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("verify");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().failure();

            // Validate the files wasn't changed
            for file in files {
                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(BROKEN_CODE, output);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_mixed() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("verify");

            let mut files = Vec::new();

            for i in 0..5 {
                let input = if i % 2 == 0 {
                    FORMATTED_CODE
                } else {
                    BROKEN_CODE
                };

                let file = setup_test_input(dir.path(), input)?;

                cmd.arg(file.path());

                files.push((file, input));
            }

            cmd.assert().failure();

            // Validate the files wasn't changed
            for (file, input) in files {
                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(input, output);
            }

            Ok(())
        }

        #[test]
        fn fails_without_input() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("verify").assert().failure();

            Ok(())
        }

        #[test]
        fn supports_log_level_argument() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--stdin")
                .arg("--log-level")
                .arg("off")
                .write_stdin(FORMATTED_CODE)
                .assert()
                .success()
                .stderr(predicates::str::is_empty());

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_prioritize_cli() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_ignore_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_ignore_config() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Ignore),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("fail")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_config() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Fail),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_fast_cli() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg("--on-missing-language-definition")
                .arg("fail-fast")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_fast_config()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("verify")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }
    }

    mod format {
        use mdsf::{
            cli::{OnMissingLanguageDefinition, OnMissingToolBinary},
            config::MdsfConfig,
            execution::MdsfFormatter,
            tools::Tooling,
        };
        use predicates::prelude::PredicateBooleanExt;

        use super::{BROKEN_CODE, setup_test_dir, setup_test_input};
        use crate::test_cli::{FORMATTED_CODE, mdsf_command};

        #[test]
        fn help_arg_outputs_message() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());

            Ok(())
        }

        #[test]
        fn formats_broken_input() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path())?;

            assert_eq!(output, FORMATTED_CODE);

            Ok(())
        }

        #[test]
        fn formats_broken_input_with_debug_arg() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--debug")
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path())?;

            assert_eq!(output, FORMATTED_CODE);

            Ok(())
        }

        #[test]
        fn formats_broken_input_stdin() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--stdin")
                .write_stdin(BROKEN_CODE)
                .assert()
                .success()
                .stdout(predicates::str::contains(FORMATTED_CODE));

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("format");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path())?;

                assert_eq!(output, FORMATTED_CODE);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_with_thread_argument()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("format").arg("--threads").arg("4");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path())?;

                assert_eq!(output, FORMATTED_CODE);
            }

            Ok(())
        }

        #[test]
        fn accepts_multiple_file_paths_with_thread_argument_zero()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let mut cmd = mdsf_command(dir.path())?;

            cmd.arg("format").arg("--threads").arg("0");

            let mut files = Vec::new();

            for _ in 0..5 {
                let file = setup_test_input(dir.path(), BROKEN_CODE)?;

                cmd.arg(file.path());

                files.push(file);
            }

            cmd.assert().success();

            for file in files {
                let output = std::fs::read_to_string(file.path())?;

                assert_eq!(output, FORMATTED_CODE);
            }

            Ok(())
        }

        #[test]
        fn format_with_cache_arg() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            for i in 0..10 {
                let mut cmd = mdsf_command(dir.path())?
                    .arg("format")
                    .arg("--cache")
                    .arg(file.path())
                    .assert()
                    .success();

                let output = std::fs::read_to_string(file.path())?;
                assert_eq!(output, FORMATTED_CODE);

                if i > 0 {
                    cmd = cmd.stderr(predicates::str::contains("(unchanged)"));
                }

                if i > 1 {
                    cmd.stderr(predicates::str::contains("(cached)"));
                }
            }

            Ok(())
        }

        #[test]
        fn fails_without_input() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?.arg("format").assert().failure();

            Ok(())
        }

        #[test]
        fn supports_log_level_argument() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--log-level")
                .arg("off")
                .assert()
                .failure()
                .stderr(predicates::str::is_empty());

            Ok(())
        }

        #[test]
        fn supports_config_path_argument() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let dir2 = tempfile::TempDir::with_prefix("mdsf")?;
            mdsf_command(dir2.path())?.arg("init").assert().success();

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--config")
                .arg(dir2.path().join("mdsf.json"))
                .arg(file.path())
                .assert()
                .success();

            let output = std::fs::read_to_string(file.path())?;
            assert_eq!(output, FORMATTED_CODE);

            Ok(())
        }

        #[test]
        fn it_should_error_if_config_is_not_valid_json() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            std::fs::write(dir.path().join("mdsf.json"), "{ this is not valid json }")?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--config")
                .arg("thisdoesnotexist.json")
                .arg(".")
                .assert()
                .failure();

            Ok(())
        }

        #[test]
        fn it_should_error_if_config_path_does_not_exist() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--config")
                .arg("thisdoesnotexist.json")
                .arg(".")
                .assert()
                .failure();

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_prioritize_cli() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_ignore_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_ignore_config() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Ignore),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("fail")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_config() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::Fail),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_fast_cli() -> Result<(), Box<dyn core::error::Error>>
        {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: None,
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-language-definition")
                .arg("fail-fast")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_language_definition_fail_fast_config()
        -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_language_definition: Some(OnMissingLanguageDefinition::FailFast),
                languages: std::collections::BTreeMap::new(),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(" no tool configured for 'rust'"));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_prioritize_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: Some(OnMissingToolBinary::FailFast),
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-tool-binary")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_ignore_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: None,
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-tool-binary")
                .arg("ignore")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_ignore_config() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: Some(OnMissingToolBinary::Ignore),
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .success()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_fail_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: None,
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-tool-binary")
                .arg("fail")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_fail_config() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: Some(OnMissingToolBinary::Fail),
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_fail_fast_cli() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: None,
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg("--on-missing-tool-binary")
                .arg("fail-fast")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }

        #[test]
        fn on_missing_tool_binary_fail_fast_config() -> Result<(), Box<dyn core::error::Error>> {
            let dir = setup_test_dir()?;

            let config = MdsfConfig {
                on_missing_tool_binary: Some(OnMissingToolBinary::FailFast),
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfFormatter::Single(Tooling::VerylFmt),
                )]),
                ..Default::default()
            };

            std::fs::write(
                dir.path().join("mdsf.json"),
                serde_json::to_string(&config)?,
            )?;

            let file = setup_test_input(dir.path(), BROKEN_CODE)?;

            mdsf_command(dir.path())?
                .arg("format")
                .arg(file.path())
                .assert()
                .failure()
                .stderr(predicates::str::contains(
                    "veryl (veryl:fmt) not found in path",
                ));

            Ok(())
        }
    }
}
