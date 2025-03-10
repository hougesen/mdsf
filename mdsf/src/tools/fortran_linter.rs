///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("fortran-linter"),
    CommandType::Uv("fortran-linter", "fortran-linter"),
    CommandType::Pipx("fortran-linter"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fortran_linter {
    #[test_with::executable(fortran-linter || pipx || uv)]
    fn test_fortran_linter_f_90_a4a8950ee39644a8() {
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

        let result = crate::tools::Tooling::FortranLinter
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
