///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_swiftformat_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("swiftformat")];

    crate::execution::run_tools(&commands, file_path, timeout, set_swiftformat_args)
}

#[cfg(test)]
mod test_swiftformat {
    #[test_with::executable(swiftformat)]
    fn test_swiftformat_swift_b40925120220f7cd() {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;
        let output = Some(
            r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("swift");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::swiftformat::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
