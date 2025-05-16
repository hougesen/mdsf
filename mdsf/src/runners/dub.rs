#[inline]
pub fn setup_dub_run_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("dub");

    cmd.arg("run");
    cmd.arg("-y");
    cmd.arg(package_name);
    cmd.arg("--");

    cmd
}

#[cfg(test)]
mod test_dub {
    #[test_with::executable(dub)]
    fn it_can_execute_d_packages() {
        let input = r#"int add(int a,int b){return a + b;}
"#;

        let output = r#"int add(int a, int b)
{
    return a + b;
}
"#;

        let file_ext = crate::filetype::get_file_extension("d");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::Dub("dfmt")],
            snippet.path(),
            crate::tools::dfmt::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::dfmt::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                dub: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
