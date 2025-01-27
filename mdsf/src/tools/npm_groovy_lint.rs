///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_npm_groovy_lint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("npm-groovy-lint"),
        CommandType::Direct("npm-groovy-lint"),
        CommandType::Npm("npm-groovy-lint"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_npm_groovy_lint_args)
}

#[cfg(test)]
mod test_npm_groovy_lint {
    #[test_with::executable(npx)]
    fn test_npm_groovy_lint_groovy_242f36b0354c3fe() {
        let input = r#"                  def add(a, b) {
            return a + b
        }

        assert add(1,2) == 3 "#;
        let output = Some(
            r#"def add(a, b) {
    return a + b
}

assert add(1, 2) == 3
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("groovy");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::npm_groovy_lint::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
