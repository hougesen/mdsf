#[inline]
pub fn setup_gem_exec_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("gem");

    cmd.arg("exec");
    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_gem_exec {
    #[test_with::executable(gem)]
    fn can_execute_command() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
  <p>
    Mads was here
  </p>
</div>
"#;

        let file_ext = crate::fttype::get_file_extension("html");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::GemExec("htmlbeautifier")],
            snippet.path(),
            crate::tools::htmlbeautifier::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::htmlbeautifier::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                gem_exec: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
