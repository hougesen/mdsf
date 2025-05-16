#[inline]
pub fn setup_pnpm_dlx_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("pnpm");

    cmd.arg("dlx").arg(package_name);

    cmd
}

#[cfg(test)]
mod test_pnpm {
    #[test_with::executable(pnpm)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "";

        let file_ext = crate::filetype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        crate::execution::run_tools(
            &[crate::runners::CommandType::Pnpm("prettier")],
            snippet.path(),
            crate::tools::prettier::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::prettier::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                pnpm: true,
                ..Default::default()
            },
        )
        .expect("it to succeed");
    }
}
