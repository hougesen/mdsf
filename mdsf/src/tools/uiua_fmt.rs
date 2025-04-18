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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("uiua")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_uiua_fmt {
    #[test_with::executable(uiua)]
    fn test_uiua_fmt_uiua_df0f003704c81512() {
        let input = r#"∿[1        5     8    2]
"#;

        let output = r#"∿[1 5 8 2]
"#;

        let file_ext = crate::fttype::get_file_extension("uiua");

        crate::tools::Tooling::UiuaFmt.test_format_snippet(input, output, &file_ext);
    }
}
