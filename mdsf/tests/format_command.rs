#[allow(dead_code)]
mod cli_command;

mod format {
    use mdsf::{
        cli::{OnMissingLanguageDefinition, OnMissingToolBinary},
        config::{MdsfConfig, MdsfTool},
        execution::MdsfFormatter,
        tools::Tooling,
    };
    use predicates::prelude::PredicateBooleanExt;

    use crate::cli_command::{
        BROKEN_CODE, FORMATTED_CODE, format_command, init_command, setup_test_dir, setup_test_input,
    };

    #[test]
    fn help_arg_outputs_message() {
        let dir = setup_test_dir();

        format_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn formats_broken_input() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .success();

        let output = std::fs::read_to_string(file.path()).unwrap();

        assert_eq!(output, FORMATTED_CODE);
    }

    #[test]
    fn formats_broken_input_with_debug_arg() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg("--debug")
            .arg(file.path())
            .assert()
            .success();

        let output = std::fs::read_to_string(file.path()).unwrap();

        assert_eq!(output, FORMATTED_CODE);
    }

    #[test]
    fn formats_broken_input_stdin() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        format_command(dir.path())
            .arg("--stdin")
            .write_stdin(BROKEN_CODE)
            .assert()
            .success()
            .stdout(predicates::str::contains(FORMATTED_CODE));
    }

    #[test]
    fn accepts_multiple_file_paths() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = format_command(dir.path());

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = format_command(dir.path());

        cmd.arg("--threads").arg("4");

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = format_command(dir.path());

        cmd.arg("--threads").arg("0");

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        for i in 0..10 {
            let mut cmd = format_command(dir.path())
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
        let dir = setup_test_dir();

        format_command(dir.path()).assert().failure();
    }

    #[test]
    fn supports_log_level_argument() {
        let dir = setup_test_dir();

        format_command(dir.path())
            .arg("--log-level")
            .arg("off")
            .assert()
            .failure()
            .stderr(predicates::str::is_empty());
    }

    #[test]
    fn supports_config_path_argument() {
        let dir = setup_test_dir();

        let dir2 = tempfile::TempDir::with_prefix("mdsf").unwrap();
        init_command(dir2.path()).assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
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
        let dir = setup_test_dir();

        std::fs::write(dir.path().join("mdsf.json"), "{ this is not valid json }").unwrap();

        format_command(dir.path())
            .arg("--config")
            .arg("thisdoesnotexist.json")
            .arg(".")
            .assert()
            .failure();
    }

    #[test]
    fn it_should_error_if_config_path_does_not_exist() {
        let dir = setup_test_dir();

        format_command(dir.path())
            .arg("--config")
            .arg("thisdoesnotexist.json")
            .arg(".")
            .assert()
            .failure();
    }

    #[test]
    fn on_missing_language_definition_prioritize_cli() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg("--on-missing-language-definition")
            .arg("ignore")
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_ignore_cli() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg("--on-missing-language-definition")
            .arg("ignore")
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_ignore_config() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_fail_cli() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg("--on-missing-language-definition")
            .arg("fail")
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_fail_config() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_fail_fast_cli() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg("--on-missing-language-definition")
            .arg("fail-fast")
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_language_definition_fail_fast_config() {
        let dir = setup_test_dir();

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

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }

    #[test]
    fn on_missing_tool_binary_prioritize_cli() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: Some(OnMissingToolBinary::FailFast),
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg("--on-missing-tool-binary")
            .arg("ignore")
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_ignore_cli() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: None,
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg("--on-missing-tool-binary")
            .arg("ignore")
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_ignore_config() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: Some(OnMissingToolBinary::Ignore),
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .success()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_fail_cli() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: None,
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg("--on-missing-tool-binary")
            .arg("fail")
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_fail_config() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: Some(OnMissingToolBinary::Fail),
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_fail_fast_cli() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: None,
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg("--on-missing-tool-binary")
            .arg("fail-fast")
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }

    #[test]
    fn on_missing_tool_binary_fail_fast_config() {
        let dir = setup_test_dir();

        let config = MdsfConfig {
            on_missing_tool_binary: Some(OnMissingToolBinary::FailFast),
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::VerylFmt)),
            )]),
            ..Default::default()
        };

        std::fs::write(
            dir.path().join("mdsf.json"),
            serde_json::to_string(&config).unwrap(),
        )
        .unwrap();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        format_command(dir.path())
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(
                "veryl (veryl:fmt) not found in path",
            ));
    }
}
