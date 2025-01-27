///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_ocamlformat_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--ignore-invalid-option");
    cmd.arg("--inplace");
    cmd.arg("--enable-outside-detected-project");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ocamlformat")];

    crate::execution::run_tools(&commands, file_path, timeout, set_ocamlformat_args)
}

#[cfg(test)]
mod test_ocamlformat {
    #[test_with::executable(ocamlformat)]
    fn test_ocamlformat_ocaml_d081ce8512af3a72() {
        let input = r#"
let add a b  =  a +  b
            "#;
        let output = Some(
            r#"let add a b = a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("ocaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ocamlformat::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
