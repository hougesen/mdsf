///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_superhtml_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("superhtml")];

    crate::execution::run_tools(&commands, file_path, timeout, set_superhtml_fmt_args)
}

#[cfg(test)]
mod test_superhtml_fmt {
    #[test_with::executable(superhtml)]
    fn test_superhtml_fmt_html_5c8e18802f77154a() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;
        let output = Some(
            r#"<div>
  <p>
    Mads was here
  </p>
</div>"#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("html");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::superhtml_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
