///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_markdownfmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("markdownfmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_markdownfmt_args)
}

#[cfg(test)]
mod test_markdownfmt {
    #[test_with::executable(markdownfmt)]
    fn test_markdownfmt_markdown_d1dd1e80b6688046() {
        let input = r#"# hello w   world

this   text has      weird spacing

- first
* second"#;
        let output = Some(
            r#"hello w world
=============

this text has weird spacing

-	first
-	second
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("markdown");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::markdownfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
