#[allow(dead_code)]
mod cli_command;

mod cache_prune {
    use predicates::prelude::PredicateBooleanExt;

    use crate::cli_command::cache_prune_command;

    #[test]
    fn help_arg_outputs_message() {
        cache_prune_command()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn removes_existing_caches() {
        cache_prune_command().assert().success();
    }

    #[test]
    fn supports_log_level_argument() {
        cache_prune_command()
            .arg("--log-level")
            .arg("off")
            .assert()
            .success()
            .stderr(predicates::str::is_empty());
    }
}
