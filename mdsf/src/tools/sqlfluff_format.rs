///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--disable-progress-bar");
    cmd.arg("--nocolor");
    cmd.arg("--dialect");
    cmd.arg("ansi");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("sqlfluff"),
    CommandType::Uv("sqlfluff", "sqlfluff"),
    CommandType::Pipx("sqlfluff"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sqlfluff_format {
    #[test_with::executable(sqlfluff || pipx || uv)]
    fn test_sqlfluff_format_sql_498b1dc9b48f9b5d() {
        let input = r#"SELECT  id                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT id FROM tbl
WHERE foo = 'bar';
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        crate::tools::Tooling::SqlfluffFormat.test_format_snippet(input, output, &file_ext);
    }
}
