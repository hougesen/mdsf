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

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
