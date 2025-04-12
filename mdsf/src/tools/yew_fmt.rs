///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--edition");
    cmd.arg("2021");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("yew-fmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_yew_fmt {
    #[test_with::executable(yew-fmt)]
    fn test_yew_fmt_rust_70ad564760e773e9() {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("rust");

        crate::tools::Tooling::YewFmt.test_format_snippet(input, output, &file_ext);
    }
}
