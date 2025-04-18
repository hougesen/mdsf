///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check");
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
mod test_kulala_fmt_check {
    #[ignore]
    #[test_with::executable(kulala-fmt || npx || pnpm || deno || bunx)]
    fn test_kulala_fmt_check_http_411ecc2948e745cf() {
        let input = r#"###


GET https://mhouge.dk HTTP/1.1
"#;

        let output = r#"###


GET https://mhouge.dk HTTP/1.1
"#;

        let file_ext = crate::fttype::get_file_extension("http");

        crate::tools::Tooling::KulalaFmtCheck.test_format_snippet(input, output, &file_ext);
    }
}
