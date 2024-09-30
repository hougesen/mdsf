use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_taplo_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_taplo(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_taplo_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_taplo(CommandType::NodeModules("taplo").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_taplo(CommandType::Direct("taplo").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_taplo(CommandType::Npm("@taplo/cli").build(), snippet_path)
}
#[cfg(test)]
mod test_taplo {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(npx)]
    fn it_should_format_toml() {
        let input = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\"
  ";

        let expected_output = "package = \"mdsf\"
author = \"Mads Hougesen\"
";

        let snippet =
            setup_snippet(input, &get_file_extension("toml")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
