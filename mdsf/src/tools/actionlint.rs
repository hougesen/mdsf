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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("actionlint")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_actionlint {
    #[test_with::executable(actionlint)]
    fn test_actionlint_yaml_da8378e9384e0b1f() {
        let input = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let file_ext = crate::fttype::get_file_extension("yaml");

        crate::tools::Tooling::Actionlint.test_format_snippet(input, output, &file_ext);
    }
}
