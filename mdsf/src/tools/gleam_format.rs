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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("gleam")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_gleam_format {
    #[test_with::executable(gleam)]
    fn test_gleam_format_gleam_d23656d11ef3a81d() {
        let input = r#"pub fn add(a:Int,b:Int)->Int{a+b}"#;

        let output = r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("gleam");

        crate::tools::Tooling::GleamFormat.test_format_snippet(input, output, &file_ext);
    }
}
