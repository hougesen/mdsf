///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd.arg("-o");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("grain")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_grain_format {
    #[test_with::executable(grain)]
    fn test_grain_format_grain_68b6e8ad56bbb476() {
        let input = r#"module Hello

                                print("Hello, world!")
"#;

        let output = r#"module Hello

print("Hello, world!")
"#;

        let file_ext = crate::fttype::get_file_extension("grain");

        crate::tools::Tooling::GrainFormat.test_format_snippet(input, output, &file_ext);
    }
}
