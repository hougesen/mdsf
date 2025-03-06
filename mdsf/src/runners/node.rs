#[inline]
pub fn new_npx_cmd(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("npx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_node {
    use crate::tools::Tooling;

    #[test_with::executable(npx)]
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
                crate::runners::JavaScriptRuntime::Node,
            )
            .expect("it to succeed");
    }
}
