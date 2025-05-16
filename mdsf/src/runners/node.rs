use super::path::setup_command_from_path;

#[inline]
pub fn setup_node_modules_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./node_modules/.bin/", binary_name)
}

#[inline]
pub fn setup_npx_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("npx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_node {
    #[test_with::executable(npx)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "";

        let file_ext = crate::filetype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        crate::execution::run_tools(
            &[crate::runners::CommandType::Npm("prettier")],
            snippet.path(),
            crate::tools::prettier::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::prettier::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                npx: true,
                ..Default::default()
            },
        )
        .expect("it to succeed");
    }
}
