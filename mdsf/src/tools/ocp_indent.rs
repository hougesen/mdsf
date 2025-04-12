///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("ocp-indent")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ocp_indent {
    #[test_with::executable(ocp-indent)]
    fn test_ocp_indent_ocaml_87a2cd7557f7a90b() {
        let input = r#"
let add a b
                             = a + b
            "#;

        let output = r#"
let add a b
  = a + b
"#;

        let file_ext = crate::fttype::get_file_extension("ocaml");

        crate::tools::Tooling::OcpIndent.test_format_snippet(input, output, &file_ext);
    }
}
