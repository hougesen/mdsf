use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_csscomb_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-t").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_csscomb(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_csscomb_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_csscomb(CommandType::NodeModules("csscomb").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_csscomb(CommandType::Direct("csscomb").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_csscomb(CommandType::Npm("csscomb").build(), snippet_path)
}

#[cfg(test)]
mod test_csscomb {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(npx)]
    fn it_should_format_css() {
        let input = "h1   {color: blue;}
p {color: red;}";

        let expected_output = "h1
{
    color: blue;
}
p
{
    color: red;
}
";

        let snippet =
            setup_snippet(input, &get_file_extension("css")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
