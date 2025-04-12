///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--html");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("xq")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_xq_html {
    #[test_with::executable(xq)]
    fn test_xq_html_html_a308d301db0ed4af() {
        let input = r#"<div>          </div>"#;

        let output = r#"<div></div>
"#;

        let file_ext = crate::fttype::get_file_extension("html");

        crate::tools::Tooling::XqHtml.test_format_snippet(input, output, &file_ext);
    }
}
