///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("xo"),
    CommandType::Direct("xo"),
    CommandType::Npm("xo"),
    CommandType::Pnpm("xo"),
    CommandType::Bun("xo"),
    CommandType::Deno("xo"),
    CommandType::Yarn("xo"),
];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_xo {
    #[test_with::executable(xo || npx || pnpm || deno || bunx)]
    fn test_xo_javascript_77a8cbfa8cbcea9d() {
        let input = r#"    function asyncAddition(a,b  )
    {
        return a+b
    }

                    console.info(asyncAddition(1, 2));"#;

        let output = r#"function asyncAddition(a, b) {
	return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::Xo.test_format_snippet(input, output, &file_ext);
    }
}
