///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_taplo_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("taplo"),
    CommandType::Direct("taplo"),
    CommandType::Npm("@taplo/cli"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_taplo_args)
}

#[cfg(test)]
mod test_taplo {
    #[test_with::executable(npx)]
    fn test_taplo_toml_34e29a1117e8cb79() {
        let input = r#"          package         =              "mdsf"
  author   = "Mads Hougesen"
  "#;
        let output = Some(
            r#"package = "mdsf"
author = "Mads Hougesen"
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("toml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::taplo::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
