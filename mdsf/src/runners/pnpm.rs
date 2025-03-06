#[inline]
pub fn new_pnpm_dlx_cmd(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("pnpm");

    cmd.arg("dlx").arg(package_name);

    cmd
}

#[cfg(test)]
mod test_pnpm {
    use crate::tools::Tooling;

    #[test_with::executable(pnpm)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "";

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        Tooling::Prettier
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                crate::runners::JavaScriptRuntime::Pnpm,
            )
            .expect("it to succeed");
    }
}
