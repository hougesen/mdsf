use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_html_beautify_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r")
        .arg("--type")
        .arg("html")
        .arg("-f")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_html_beautify(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_html_beautify_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_html_beautify(
        CommandType::NodeModules("html-beautify").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_html_beautify(CommandType::Direct("html-beautify").build(), snippet_path)
}

#[cfg(test)]
mod test_html_beautify {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(html-beautify)]
    fn it_should_format_css() {
        let input = "<div>
                    <p>
                    Mads was here
                    </p>
        </div>";

        let expected_output = "<div>
    <p>
        Mads was here
    </p>
</div>";

        let snippet =
            setup_snippet(input, language_to_ext("html")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
