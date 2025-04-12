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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("sql-formatter"),
    CommandType::Direct("sql-formatter"),
    CommandType::Npm("sql-formatter"),
    CommandType::Pnpm("sql-formatter"),
    CommandType::Bun("sql-formatter"),
    CommandType::Deno("sql-formatter"),
    CommandType::Yarn("sql-formatter"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sql_formatter {
    #[test_with::executable(sql-formatter || npx || pnpm || deno || bunx)]
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

        crate::tools::Tooling::SqlFormatter.test_format_snippet(input, output, &file_ext);
    }
}
