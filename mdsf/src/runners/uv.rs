#[inline]
pub fn setup_uv_run_command(package_name: &str, executable_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("uv");

    cmd.arg("tool");
    cmd.arg("run");
    cmd.arg("--no-progress");

    if package_name == executable_name {
        cmd.arg(package_name);
    } else {
        cmd.arg("--from");
        cmd.arg(package_name);
        cmd.arg(executable_name);
    }

    cmd
}

#[cfg(test)]
mod test_uv {
    #[test_with::executable(uv)]
    fn it_works_without_executable_name() {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example"#;

        let output = r#"program example
implicit none (type, external)

contains
integer function addnum(a, b)
    integer, intent(in) :: a, b
    return a + b
end function addnum

end program example
"#;

        let file_ext = crate::filetype::get_file_extension(".f90");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::Uv(
                "fortran-linter",
                "fortran-linter",
            )],
            snippet.path(),
            crate::tools::fortran_linter::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::fortran_linter::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                uv: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(uv)]
    fn it_works_with_executable_name() {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>"#;

        let file_ext = crate::filetype::get_file_extension("xml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::Uv("xmlformatter", "xmlformat")],
            snippet.path(),
            crate::tools::xmlformat::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::xmlformat::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                uv: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
