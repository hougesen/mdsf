///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_css_beautify_args(
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

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("css-beautify"),
    CommandType::Direct("css-beautify"),
    CommandType::Npm("js-beautify"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_css_beautify_args)
}

#[cfg(test)]
mod test_css_beautify {
    #[test_with::executable(npx)]
    fn test_css_beautify_css_9037e3781d1e74dd() {
        let input = r#"h1   {color: blue;} p    {color: red;}"#;
        let output = Some(
            r#"h1 {
    color: blue;
}

p {
    color: red;
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("css");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::css_beautify::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
