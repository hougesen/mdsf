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
mod test_biome_format {
    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_format_javascript_4845e9b01c23667f() {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
	return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::BiomeFormat.test_format_snippet(input, output, &file_ext);
    }

    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_format_json_90a326e29048e3cd() {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
	"key": "value",
	"key2": ["value2", "value3", 1, null]
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        crate::tools::Tooling::BiomeFormat.test_format_snippet(input, output, &file_ext);
    }

    #[test_with::executable(biome || npx || pnpm || deno || bunx)]
    fn test_biome_format_typescript_8154bfdbd3b72275() {
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

        crate::tools::Tooling::BiomeFormat.test_format_snippet(input, output, &file_ext);
    }
}
