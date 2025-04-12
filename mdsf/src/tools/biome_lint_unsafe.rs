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
    cmd.arg("--unsafe");
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
mod test_biome_lint_unsafe {
    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_lint_unsafe_javascript_9165f2e512bbc53f() {
        let input = r#"const hello = "hello";
const world = "world";

console.log("" + hello + world);
"#;

        let output = r#"const hello = "hello";
const world = "world";

console.log(`${hello}${world}`);
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::BiomeLintUnsafe.test_format_snippet(input, output, &file_ext);
    }
}
