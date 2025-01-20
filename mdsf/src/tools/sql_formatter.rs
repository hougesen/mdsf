///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_sql_formatter_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("sql-formatter"),
        CommandType::Direct("sql-formatter"),
        CommandType::Npm("sql-formatter"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_sql_formatter_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

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
        let result = crate::tools::sql_formatter::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
