///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_yamlfmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("yamlfmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_yamlfmt_args)
}

#[cfg(test)]
mod test_yamlfmt {
    #[test_with::executable(yamlfmt)]
    fn test_yamlfmt_yaml_f383efd52f3fcc71() {
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
        let output = Some(
            r#"version: 2
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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("yaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::yamlfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
