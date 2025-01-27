///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_just_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--fmt");
    cmd.arg("--unstable");
    cmd.arg("--justfile");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("just"),
        CommandType::Direct("just"),
        CommandType::Npm("rust-just"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_just_args)
}

#[cfg(test)]
mod test_just {
    #[test_with::executable(npx)]
    fn test_just_just_9737c58292992524() {
        let input = r#"build:
                cargo build
                cargo build --release
            "#;
        let output = Some(
            r#"build:
    cargo build
    cargo build --release
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("just");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::just::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
