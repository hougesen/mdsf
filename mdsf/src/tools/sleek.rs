///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("sleek")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sleek {
    #[test_with::executable(sleek)]
    fn test_sleek_sql_d16819f4564d8853() {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT
    *
FROM
    tbl
WHERE
    foo = 'bar';"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        crate::tools::Tooling::Sleek.test_format_snippet(input, output, &file_ext);
    }
}
