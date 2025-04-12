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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("ktfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ktfmt {
    #[test_with::executable(ktfmt)]
    fn test_ktfmt_kotlin_434b08006e9b780a() {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"fun add(a: Int, b: Int): Int {
  return a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("kotlin");

        crate::tools::Tooling::Ktfmt.test_format_snippet(input, output, &file_ext);
    }
}
