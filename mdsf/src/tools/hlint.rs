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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("hlint")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_hlint {
    #[test_with::executable(hlint)]
    fn test_hlint_haskell_ea6b440c7b6ee01d() {
        let input = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let output = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let file_ext = crate::fttype::get_file_extension("haskell");

        crate::tools::Tooling::Hlint.test_format_snippet(input, output, &file_ext);
    }
}
