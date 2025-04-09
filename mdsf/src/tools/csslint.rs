///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("csslint"),
    CommandType::Direct("csslint"),
    CommandType::Npm("csslint"),
    CommandType::Pnpm("csslint"),
    CommandType::Bun("csslint"),
    CommandType::Deno("csslint"),
    CommandType::Yarn("csslint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_csslint {
    #[test_with::executable(csslint || npx || pnpm || deno || bunx)]
    fn test_csslint_css_9b7fd0554eb344f() {
        let input = r#"body {
  background: red;
}
"#;

        let output = r#"body {
  background: red;
}
"#;

        let file_ext = crate::fttype::get_file_extension("css");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Csslint
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
