///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("taplo"),
    CommandType::Direct("taplo"),
    CommandType::Npm("@taplo/cli"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_taplo {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
    fn test_taplo_toml_f9c7870e88d1963c() {
        let input = r#"          package         =              "mdsf"
  author   = "Mads Hougesen"
  "#;

        let output = r#"package = "mdsf"
author = "Mads Hougesen"
"#;

        let file_ext = crate::fttype::get_file_extension("toml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
