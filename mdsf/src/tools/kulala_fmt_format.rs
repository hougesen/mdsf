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
    CommandType::NodeModules("kulala-fmt"),
    CommandType::Direct("kulala-fmt"),
    CommandType::Npm("@mistweaverco/kulala-fmt"),
    CommandType::Pnpm("@mistweaverco/kulala-fmt"),
    CommandType::Bun("@mistweaverco/kulala-fmt"),
    CommandType::Deno("@mistweaverco/kulala-fmt"),
    CommandType::Yarn("@mistweaverco/kulala-fmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_kulala_fmt_format {
    #[ignore]
    #[test_with::executable(kulala-fmt || npx || pnpm || deno || bunx)]
    fn test_kulala_fmt_format_http_51b00899e0a1c815() {
        let input = r#"GET          https://mhouge.dk          "#;

        let output = r#"###


GET https://mhouge.dk HTTP/1.1
"#;

        let file_ext = crate::fttype::get_file_extension("http");

        crate::tools::Tooling::KulalaFmtFormat.test_format_snippet(input, output, &file_ext);
    }
}
