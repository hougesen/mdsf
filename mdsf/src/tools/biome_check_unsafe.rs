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
    cmd.arg("--write");
    cmd.arg("--unsafe");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("biome"),
    CommandType::Direct("biome"),
    CommandType::Npm("@biomejs/biome"),
    CommandType::Pnpm("@biomejs/biome"),
    CommandType::Bun("@biomejs/biome"),
    CommandType::Deno("@biomejs/biome"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_biome_check_unsafe {
    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_check_unsafe_typescript_8154bfdbd3b72275() {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("typescript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::BiomeCheckUnsafe
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
