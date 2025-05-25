#[cfg(test)]
mod test_cli {
    use predicates::prelude::PredicateBooleanExt;

    #[test]
    fn help_arg_outputs_message() {
        assert_cmd::Command::cargo_bin("mdsf")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn version_arg_outputs_message() {
        assert_cmd::Command::cargo_bin("mdsf")
            .unwrap()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    mod help {
        use predicates::prelude::PredicateBooleanExt;

        #[test]
        fn command_outputs_help_message() {
            assert_cmd::Command::cargo_bin("mdsf")
                .unwrap()
                .arg("help")
                .assert()
                .success()
                .stdout(predicates::str::is_empty().not());
        }
    }
}
