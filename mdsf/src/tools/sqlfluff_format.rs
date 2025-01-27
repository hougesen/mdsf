///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_sqlfluff_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg("--dialect");
    cmd.arg("ansi");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("sqlfluff")];

    crate::execution::run_tools(&commands, file_path, timeout, set_sqlfluff_format_args)
}

#[cfg(test)]
mod test_sqlfluff_format {
    #[test_with::executable(sqlfluff)]
    fn test_sqlfluff_format_sql_16c512917bb843b9() {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;
        let output = Some(
            r#"SELECT * FROM tbl
WHERE foo = 'bar';
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("sql");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::sqlfluff_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
