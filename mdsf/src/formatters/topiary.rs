// NOTE: queries must be setup locally
// export TOPIARY_LANGUAGE_DIR=/home/houge/Desktop/github/tweag/topiary/topiary-queries/queries/

use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_topiary_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_topiary(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_topiary_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_topiary(CommandType::Direct("topiary").build(), snippet_path)
}

#[cfg(test)]
mod test_topiary {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(topiary)]
    fn it_should_format_json() {
        let input = "
              {
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

        let expected_output = "{
  \"key\": \"value\",
  \"key2\": [
    \"value2\",
    \"value3\",
    1,
    null
  ]
}
";

        let snippet =
            setup_snippet(input, &get_file_extension("json")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
