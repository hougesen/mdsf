///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix");
    cmd.arg("--force");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("sqruff"),
    CommandType::Uv("sqruff", "sqruff"),
    CommandType::Pipx("sqruff"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sqruff {
    #[test_with::executable(sqruff || pipx || uv)]
    fn test_sqruff_sql_c48780a07bf33db() {
        let input = r#"SELECT          * from dummy where Name     > 10
"#;

        let output = r#"SELECT * FROM dummy WHERE name > 10
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Sqruff
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
