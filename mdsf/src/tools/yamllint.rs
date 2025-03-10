///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("yamllint"),
    CommandType::Uv("yamllint", "yamllint"),
    CommandType::Pipx("yamllint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_yamllint {
    #[test_with::executable(yamllint || pipx || uv)]
    fn test_yamllint_yaml_e7ca97ee9ae56e12() {
        let input = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let file_ext = crate::fttype::get_file_extension("yaml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Yamllint
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
