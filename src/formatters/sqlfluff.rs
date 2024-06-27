use super::execute_command;
use crate::error::MdsfError;

#[inline]
fn set_sqlfluff_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("format")
        .arg("--dialect")
        // TODO: custom dialect?
        .arg("ansi")
        .arg(snippet_path);
}

#[inline]
async fn invoke_sqlfluff(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_sqlfluff_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_sqlfluff(tokio::process::Command::new("sqlfluff"), snippet_path).await
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
