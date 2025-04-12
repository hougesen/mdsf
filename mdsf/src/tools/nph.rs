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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("nph")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_nph {
    #[test_with::executable(nph)]
    fn test_nph_nim_4f0f6867e821c18d() {
        let input = r#"proc add(a:int,b:int):int = 
            return a+b
"#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("nim");

        crate::tools::Tooling::Nph.test_format_snippet(input, output, &file_ext);
    }
}
