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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("nimpretty")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_nimpretty {
    #[test_with::executable(nimpretty)]
    fn test_nimpretty_nim_2c41c79e1d74972a() {
        let input = r#"proc           add( a         :int , b:int )        : int =
  return a +          b  "#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("nim");

        crate::tools::Tooling::Nimpretty.test_format_snippet(input, output, &file_ext);
    }
}
