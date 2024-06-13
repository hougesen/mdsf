use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_taplo_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("fmt");
    cmd.arg(snippet_path);
}

#[inline]
fn invoke_taplo(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_taplo_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_taplo(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_taplo(std::process::Command::new("taplo"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_taplo(setup_npm_script("@taplo/cli"), snippet_path)
}
#[cfg(test)]
mod test_taplo {
    use crate::{
        formatters::{setup_snippet, taplo::format_using_taplo},
        generated::language_to_ext,
    };

    #[test_with::executable(npx)]
    fn it_should_format_toml() {
        let input = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\"
  ";

        let expected_output = "package = \"mdsf\"
author = \"Mads Hougesen\"
";

        let snippet =
            setup_snippet(input, &language_to_ext("toml")).expect("it to create a snippet file");

        let output = format_using_taplo(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
