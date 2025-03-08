#[inline]
pub fn setup_deno_run_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("deno");

    cmd.arg("run");
    cmd.arg("-A");
    cmd.arg(format!("npm:{package_name}"));

    cmd
}

#[cfg(test)]
mod test_deno {
    #[test_with::executable(deno)]
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
                    deno: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .expect("it to succeed");
    }
}
