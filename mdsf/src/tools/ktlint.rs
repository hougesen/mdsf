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
    #[test_with::executable(ktlint)]
    fn test_ktlint_kotlin_3421435c9e766a31() {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"

fun add(
    a: Int,
    b: Int,
): Int = a + b
"#;

        let file_ext = crate::fttype::get_file_extension("kotlin");

        crate::tools::Tooling::Ktlint.test_format_snippet(input, output, &file_ext);
    }
}
