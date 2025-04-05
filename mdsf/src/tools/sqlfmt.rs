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

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Sqlfmt
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
