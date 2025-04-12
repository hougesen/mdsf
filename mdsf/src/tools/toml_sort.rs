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

        crate::tools::Tooling::TomlSort.test_format_snippet(input, output, &file_ext);
    }
}
