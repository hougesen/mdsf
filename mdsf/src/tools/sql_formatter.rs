///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("sql-formatter"),
    CommandType::Direct("sql-formatter"),
    CommandType::Npm("sql-formatter"),
];

#[cfg(test)]
mod test_sql_formatter {
    #[test_with::executable(npx)]
    fn test_sql_formatter_sql_85ac36a4bf14f957() {
        let input = r#"SELECT * FROM tbl WHERE foo = 'bar';"#;

        let output = r#"SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_args, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}
