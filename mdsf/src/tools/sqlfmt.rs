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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("sqlfmt"),
    CommandType::Uv("shandy-sqlfmt[jinjafmt]", "sqlfmt"),
    CommandType::Pipx("shandy-sqlfmt[jinjafmt]"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sqlfmt {
    #[test_with::executable(sqlfmt || pipx || uv)]
    fn test_sqlfmt_sql_7933045821741e3() {
        let input = r#"SELECT  *                  FROM  tbl                        WHERE                      foo   = 'bar';"#;

        let output = r#"select *
from tbl
where foo = 'bar'
;
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        crate::tools::Tooling::Sqlfmt.test_format_snippet(input, output, &file_ext);
    }
}
