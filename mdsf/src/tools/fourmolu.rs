///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("fourmolu")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fourmolu {
    #[test_with::executable(fourmolu)]
    fn test_fourmolu_haskell_718612a8aa064d19() {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
"#;

        let file_ext = crate::fttype::get_file_extension("haskell");

        crate::tools::Tooling::Fourmolu.test_format_snippet(input, output, &file_ext);
    }
}
