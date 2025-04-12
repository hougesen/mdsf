///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--ignore-invalid-option");
    cmd.arg("--inplace");
    cmd.arg("--enable-outside-detected-project");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("ocamlformat")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ocamlformat {
    #[test_with::executable(ocamlformat)]
    fn test_ocamlformat_ocaml_5f599d285848218() {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let file_ext = crate::fttype::get_file_extension("ocaml");

        crate::tools::Tooling::Ocamlformat.test_format_snippet(input, output, &file_ext);
    }
}
