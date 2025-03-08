#[inline]
pub fn setup_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("pipx");

    cmd.arg("run");
    cmd.arg("--quiet");
    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_pipx {
    #[test_with::executable(pipx)]
    fn test_can_execute_command() {
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

        let file_ext = crate::fttype::get_file_extension(".f90");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::PipxRun("fortran-linter")],
            snippet.path(),
            crate::tools::fortran_linter::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::fortran_linter::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                python: Some(crate::config::MdsfConfigRunnersPython {
                    pipx: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
