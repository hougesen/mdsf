#[allow(dead_code)]
mod cli_command;

mod verify {
    use mdsf::{cli::OnMissingLanguageDefinition, config::MdsfConfig};
    use predicates::prelude::PredicateBooleanExt;

    use crate::cli_command::{
        BROKEN_CODE, FORMATTED_CODE, init_command, setup_test_dir, setup_test_input, verify_command,
    };

    #[test]
    fn help_arg_outputs_message() {
        let dir = setup_test_dir();

        verify_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn success_with_formatted_input() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), FORMATTED_CODE);

        verify_command(dir.path())
            .arg(file.path())
            .assert()
            .success();
    }

    #[test]
    fn success_with_formatted_input_and_debug_arg() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), FORMATTED_CODE);

        verify_command(dir.path())
            .arg("--debug")
            .arg(file.path())
            .assert()
            .success();
    }

    #[test]
    fn success_with_formatted_input_stdin() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        verify_command(dir.path())
            .arg("--stdin")
            .write_stdin(FORMATTED_CODE)
            .assert()
            .success();
    }

    #[test]
    fn fails_with_broken_input() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let file = setup_test_input(dir.path(), BROKEN_CODE);

        verify_command(dir.path())
            .arg(file.path())
            .assert()
            .failure();
    }

    #[test]
    fn fails_with_broken_input_stdin() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        verify_command(dir.path())
            .arg("--stdin")
            .write_stdin(BROKEN_CODE)
            .assert()
            .failure();
    }

    #[test]
    fn accepts_multiple_file_paths_formatted() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = verify_command(dir.path());

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = verify_command(dir.path());

        cmd.arg("--threads").arg("4");

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = verify_command(dir.path());

        cmd.arg("--threads").arg("0");

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = verify_command(dir.path());

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
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let mut cmd = verify_command(dir.path());

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
        let dir = setup_test_dir();

        verify_command(dir.path()).assert().failure();
    }

    #[test]
    fn supports_log_level_argument() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
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

        verify_command(dir.path())
            .arg(file.path())
            .assert()
            .failure()
            .stderr(predicates::str::contains(" no tool configured for 'rust'"));
    }
}
