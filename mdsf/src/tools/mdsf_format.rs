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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("mdsf"),
    CommandType::Direct("mdsf"),
    CommandType::Npm("mdsf-cli"),
    CommandType::Pnpm("mdsf-cli"),
    CommandType::Bun("mdsf-cli"),
    CommandType::Deno("mdsf-cli"),
    CommandType::Yarn("mdsf-cli"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mdsf_format {
    #[test_with::executable(mdsf || npx || pnpm || deno || bunx)]
    fn test_mdsf_format_markdown_1e1586f943958589() {
        let input = r#""#;

        let output = r#""#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::MdsfFormat.test_format_snippet(input, output, &file_ext);
    }
}
