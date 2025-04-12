///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-t");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("csscomb"),
    CommandType::Direct("csscomb"),
    CommandType::Npm("csscomb"),
    CommandType::Pnpm("csscomb"),
    CommandType::Bun("csscomb"),
    CommandType::Deno("csscomb"),
    CommandType::Yarn("csscomb"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_csscomb {
    #[test_with::executable(csscomb || npx || pnpm || deno || bunx)]
    fn test_csscomb_css_bed67a883a4a1aae() {
        let input = r#"h1   {color: blue;}
p {color: red;}"#;

        let output = r#"h1
{
    color: blue;
}
p
{
    color: red;
}
"#;

        let file_ext = crate::fttype::get_file_extension("css");

        crate::tools::Tooling::Csscomb.test_format_snippet(input, output, &file_ext);
    }
}
