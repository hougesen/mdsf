use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_css_beautify_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r")
        .arg("--type")
        .arg("css")
        .arg("-f")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_css_beautify(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_css_beautify_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_css_beautify(
        CommandType::NodeModules("css-beautify").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_css_beautify(CommandType::Direct("css-beautify").build(), snippet_path)
}

#[cfg(test)]
mod test_css_beautify {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(css-beautify)]
    fn it_should_format_css() {
        let input = "h1   {color: blue;} p    {color: red;}";

        let expected_output = "h1 {
    color: blue;
}

p {
    color: red;
}";

        let snippet =
            setup_snippet(input, language_to_ext("css")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
