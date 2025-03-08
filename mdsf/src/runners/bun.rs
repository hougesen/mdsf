#[inline]
pub fn setup_bunx_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("bunx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_bun {
    #[test_with::executable(bunx)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "";

        let file_ext = crate::fttype::get_file_extension("json");

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
                javascript: Some(crate::config::MdsfConfigRunnersJavaScript {
                    bunx: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .expect("it to succeed");
    }
}
