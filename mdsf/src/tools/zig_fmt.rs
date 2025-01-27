///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_zig_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("zig")];

    crate::execution::run_tools(&commands, file_path, timeout, set_zig_fmt_args)
}

#[cfg(test)]
mod test_zig_fmt {
    #[test_with::executable(zig)]
    fn test_zig_fmt_zig_4e17c63c72d89acb() {
        let input = r#"
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    "#;
        let output = Some(
            r#"fn add(a: i32, b: i32) i32 {
    return a + b;
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("zig");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::zig_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
