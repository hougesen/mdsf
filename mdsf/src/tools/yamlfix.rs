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
    CommandType::Direct("yamlfix"),
    CommandType::Uv("yamlfix", "yamlfix"),
    CommandType::Pipx("yamlfix"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_yamlfix {
    #[test_with::executable(yamlfix || pipx || uv)]
    fn test_yamlfix_yaml_9fcbc943bcaf9d7f() {
        let input = r#"


version:                                                                             2
updates:
  - package-ecosystem:                    "cargo"
    directory:  "/"
    schedule:
      interval:     "monthly"
    assignees:
      -     "hougesen"
    open-pull-requests-limit:       25

  - package-ecosystem:                              "github-actions"
    directory:          "/"
    schedule:
        interval:          "monthly"
    assignees:
        - "hougesen"
    open-pull-requests-limit: 25


        "#;

        let output = r#"---
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
"#;

        let file_ext = crate::fttype::get_file_extension("yaml");

        crate::tools::Tooling::Yamlfix.test_format_snippet(input, output, &file_ext);
    }
}
