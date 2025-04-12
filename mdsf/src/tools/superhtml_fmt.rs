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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("superhtml")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_superhtml_fmt {
    #[test_with::executable(superhtml)]
    fn test_superhtml_fmt_html_8183dae6d1f190e1() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
  <p>
    Mads was here
  </p>
</div>"#;

        let file_ext = crate::fttype::get_file_extension("html");

        crate::tools::Tooling::SuperhtmlFmt.test_format_snippet(input, output, &file_ext);
    }
}
