///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("yamlfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_yamlfmt {
    #[test_with::executable(yamlfmt)]
    fn test_yamlfmt_yaml_5f37046bfdc59220() {
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

        let output = r#"version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
"#;

        let file_ext = crate::fttype::get_file_extension("yaml");

        crate::tools::Tooling::Yamlfmt.test_format_snippet(input, output, &file_ext);
    }
}
