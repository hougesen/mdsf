///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("toml-sort"),
    CommandType::Uv("toml-sort", "toml-sort"),
    CommandType::Pipx("toml-sort"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_toml_sort {
    #[test_with::executable(toml-sort || pipx || uv)]
    fn test_toml_sort_toml_8c2b58a6580e9412() {
        let input = r#"

[c]
key = "something"


[a]
key = "something"

[b]
key = "something"

"#;

        let output = r#"[a]
key = "something"

[b]
key = "something"

[c]
key = "something"
"#;

        let file_ext = crate::fttype::get_file_extension("toml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::TomlSort
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
