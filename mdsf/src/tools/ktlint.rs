///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--format");
    cmd.arg("--log-level=error");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("ktlint")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ktlint {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(ktlint)]
    fn test_ktlint_kotlin_65d99b8b0b9bf8e1() {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"

fun add(
    a: Int,
    b: Int,
): Int {
    return a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("kotlin");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
