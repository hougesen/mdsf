///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_sql_formatter_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("sql-formatter"),
    CommandType::Direct("sql-formatter"),
    CommandType::Npm("sql-formatter"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_sql_formatter_args)
}

#[cfg(test)]
mod test_sql_formatter {
    #[test_with::executable(npx)]
    fn test_sql_formatter_sql_3d92b902cf65ff90() {
        let input = r#"SELECT * FROM tbl WHERE foo = 'bar';"#;
        let output = Some(
            r#"SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("sql");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::sql_formatter::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
