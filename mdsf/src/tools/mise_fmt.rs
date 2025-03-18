///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("mise"),
    CommandType::Direct("mise"),
    CommandType::Npm("@jdxcode/mise"),
    CommandType::Pnpm("@jdxcode/mise"),
    CommandType::Bun("@jdxcode/mise"),
    CommandType::Deno("@jdxcode/mise"),
    CommandType::Yarn("@jdxcode/mise"),
];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_mise_fmt {
    #[test_with::executable(mise || npx || pnpm || deno || bunx)]
    fn test_mise_fmt_toml_7a3c9e91cda91a26() {
        let input = r#"[env]
NODE_ENV = 'production'


[tools]
erlang                = ['23.3', '24.0']
terraform = '1.0.0'














[tasks.build]
run = 'echo "running build tasks"'
"#;

        let output = r#"[env]
NODE_ENV = 'production'


[tools]
erlang = ['23.3', '24.0']
terraform = '1.0.0'


[tasks.build]
run = 'echo "running build tasks"'
"#;

        let file_ext = crate::fttype::get_file_extension("toml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::MiseFmt
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
