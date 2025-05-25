#[allow(dead_code)]
mod cli_command;

mod init {
    use predicates::prelude::PredicateBooleanExt;

    use crate::cli_command::{init_command, setup_test_dir};

    #[test]
    fn help_arg_outputs_message() {
        let dir = setup_test_dir();

        init_command(dir.path())
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn creates_a_config_file() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        let config_file_created = dir.path().join("mdsf.json").try_exists().unwrap();

        assert!(config_file_created);
    }

    #[test]
    fn fails_if_config_exists() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        init_command(dir.path()).assert().failure();
    }

    #[test]
    fn force_config_arg() {
        let dir = setup_test_dir();

        init_command(dir.path()).assert().success();

        init_command(dir.path()).assert().failure();

        init_command(dir.path()).arg("--force").assert().success();
    }

    #[test]
    fn supports_log_level_argument() {
        let dir = setup_test_dir();

        init_command(dir.path())
            .arg("--log-level")
            .arg("off")
            .assert()
            .success()
            .stderr(predicates::str::is_empty());
    }
}
