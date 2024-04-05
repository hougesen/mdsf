use super::execute_command;

#[inline]
fn set_sqlfluff_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("format")
        .arg("--dialect")
        // TODO: custom dialect?
        .arg("ansi")
        .arg(snippet_path);
}

#[inline]
fn invote_sqlfluff(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    set_sqlfluff_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_sqlfluff(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    invote_sqlfluff(std::process::Command::new("sqlfluff"), snippet_path)
}

#[cfg(test)]
mod test_sqlfluff {
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(sqlfluff)]
    #[test]
    fn it_should_format_sql() {
        let input = "SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         ";

        let expected_output = "SELECT * FROM tbl
WHERE foo = 'bar';
";

        let snippet =
            setup_snippet(input, Language::Sql.to_file_ext()).expect("it to create a snippet file");

        let output = super::format_using_sqlfluff(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
