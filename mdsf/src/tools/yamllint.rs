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

        crate::tools::Tooling::Yamllint.test_format_snippet(input, output, &file_ext);
    }
}
