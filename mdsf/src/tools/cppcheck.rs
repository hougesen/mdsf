///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("cppcheck")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_cppcheck {
    #[test_with::executable(cppcheck)]
    fn test_cppcheck_cpp_fd936e483242a65d() {
        let input = r#"int add(int a, int b) { return a + b; }
"#;

        let output = r#"int add(int a, int b) { return a + b; }
"#;

        let file_ext = crate::fttype::get_file_extension("cpp");

        crate::tools::Tooling::Cppcheck.test_format_snippet(input, output, &file_ext);
    }
}
