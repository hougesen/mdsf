///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint");
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
mod test_sqlfluff_lint {
    #[test_with::executable(sqlfluff || pipx || uv)]
    fn test_sqlfluff_lint_sql_9ec8d0c58d20cc30() {
        let input = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let output = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        crate::tools::Tooling::SqlfluffLint.test_format_snippet(input, output, &file_ext);
    }
}
