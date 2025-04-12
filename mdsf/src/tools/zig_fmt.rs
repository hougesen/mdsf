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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("zig")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_zig_fmt {
    #[test_with::executable(zig)]
    fn test_zig_fmt_zig_8151c333113cef41() {
        let input = r#"
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    "#;

        let output = r#"fn add(a: i32, b: i32) i32 {
    return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("zig");

        crate::tools::Tooling::ZigFmt.test_format_snippet(input, output, &file_ext);
    }
}
