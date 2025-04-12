///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("biome"),
    CommandType::Direct("biome"),
    CommandType::Npm("@biomejs/biome"),
    CommandType::Pnpm("@biomejs/biome"),
    CommandType::Bun("@biomejs/biome"),
    CommandType::Deno("@biomejs/biome"),
    CommandType::Yarn("@biomejs/biome"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_biome_lint {
    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_lint_javascript_3b1c1d6fd9c2e176() {
        let input = r#"let variable = 0;
"#;

        let output = r#"const variable = 0;
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::BiomeLint.test_format_snippet(input, output, &file_ext);
    }
}
