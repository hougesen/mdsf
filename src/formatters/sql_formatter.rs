use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_sql_formatter_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
async fn invoke_sql_formatter(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_sql_formatter_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_sql_formatter(tokio::process::Command::new("sql-formatter"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_sql_formatter(setup_npm_script("sql-formatter"), snippet_path).await
}

#[cfg(test)]
mod test_sql_formatter {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_sql() {
        let input = "SELECT * FROM tbl WHERE foo = 'bar';";

        let expected_output = "SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
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
