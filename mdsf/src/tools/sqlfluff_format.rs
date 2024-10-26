use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_sqlfluff_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg("--dialect");
    cmd.arg("ansi");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("sqlfluff")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_sqlfluff_format_args(cmd.build(), file_path);
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
mod test_sqlfluff_format {
    #[test_with::executable(sqlfluff)]
    fn test_sqlfluff_format_sql_7d0d03aa13961f14() {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;
        let output = r#"SELECT * FROM tbl
WHERE foo = 'bar';
"#;
        let file_ext = crate::fttype::get_file_extension("sql");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::sqlfluff_format::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
