use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_sqlfluff_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format")
        .arg("--dialect")
        // TODO: custom dialect?
        .arg("ansi")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_sqlfluff(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_sqlfluff_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_sqlfluff(CommandType::Direct("sqlfluff").build(), snippet_path)
}

#[cfg(test)]
mod test_sqlfluff {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(sqlfluff)]
    fn it_should_format_sql() {
        let input = "SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         ";

        let expected_output = "SELECT * FROM tbl
WHERE foo = 'bar';
";

        let snippet =
            setup_snippet(input, language_to_ext("sql")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
