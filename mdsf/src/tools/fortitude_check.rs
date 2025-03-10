///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check");
    cmd.arg("--quiet");
    cmd.arg("--no-respect-gitignore");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("fortitude"),
    CommandType::Uv("fortitude-lint", "fortitude"),
    CommandType::Pipx("fortitude-lint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fortitude_check {
    #[test_with::executable(fortitude || pipx || uv)]
    fn test_fortitude_check_f_90_3b0b8d0e32ad7855() {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

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

        let result = crate::tools::Tooling::FortitudeCheck
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
