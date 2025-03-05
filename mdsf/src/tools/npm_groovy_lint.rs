///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("npm-groovy-lint"),
    CommandType::Direct("npm-groovy-lint"),
    CommandType::Npm("npm-groovy-lint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_npm_groovy_lint {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
    fn test_npm_groovy_lint_groovy_2dc2be09d8013576() {
        let input = r#"                  def add(a, b) {
            return a + b
        }

        assert add(1,2) == 3 "#;

        let output = r#"def add(a, b) {
    return a + b
}

assert add(1, 2) == 3
"#;

        let file_ext = crate::fttype::get_file_extension("groovy");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
