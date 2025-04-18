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

pub const COMMANDS: [CommandType; 2] = [
    CommandType::Direct("htmlbeautifier"),
    CommandType::GemExec("htmlbeautifier"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_htmlbeautifier {
    #[test_with::executable(htmlbeautifier || gem)]
    fn test_htmlbeautifier_html_7e86d833d3fbf4e3() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
  <p>
    Mads was here
  </p>
</div>
"#;

        let file_ext = crate::fttype::get_file_extension("html");

        crate::tools::Tooling::Htmlbeautifier.test_format_snippet(input, output, &file_ext);
    }
}
