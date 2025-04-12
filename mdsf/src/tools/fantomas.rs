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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("fantomas")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fantomas {
    #[test_with::executable(fantomas)]
    fn test_fantomas_fsharp_f3cb7f290d0660d3() {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let file_ext = crate::fttype::get_file_extension("fsharp");

        crate::tools::Tooling::Fantomas.test_format_snippet(input, output, &file_ext);
    }
}
