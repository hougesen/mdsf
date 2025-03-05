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
    cmd.arg("--dialect");
    cmd.arg("ansi");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("sqlfluff")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_sqlfluff_format {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(sqlfluff)]
    fn test_sqlfluff_format_sql_55c68b000536eccf() {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT * FROM tbl
WHERE foo = 'bar';
"#;

        let file_ext = crate::fttype::get_file_extension("sql");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
