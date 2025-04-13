///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("--all-code");
    cmd.arg("--file");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("dx")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_dx_fmt {
    #[test_with::executable(dx)]
    fn test_dx_fmt_rust_c07936252118b5c6() {
        let input = r#"fn add(a:i32,b:i32)->i32{a+b}"#;

        let output = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("rust");

        crate::tools::Tooling::DxFmt.test_format_snippet(input, output, &file_ext);
    }
}
