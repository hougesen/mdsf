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
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

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
