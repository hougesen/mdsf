///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("css");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("css-beautify"),
    CommandType::Direct("css-beautify"),
    CommandType::Npm("js-beautify"),
];

#[cfg(test)]
mod test_css_beautify {
    #[test_with::executable(npx)]
    fn test_css_beautify_css_5ad41f26f69aea3e() {
        let input = r#"h1   {color: blue;} p    {color: red;}"#;

        let output = r#"h1 {
    color: blue;
}

p {
    color: red;
}"#;

        let file_ext = crate::fttype::get_file_extension("css");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_args, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}
