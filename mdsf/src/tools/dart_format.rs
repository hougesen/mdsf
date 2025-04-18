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
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("dart")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_dart_format {
    #[test_with::executable(dart)]
    fn test_dart_format_dart_1e68d7619b4be391() {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;

        let output = r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#;

        let file_ext = crate::fttype::get_file_extension("dart");

        crate::tools::Tooling::DartFormat.test_format_snippet(input, output, &file_ext);
    }
}
