///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_htmlbeautifier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("htmlbeautifier")];

    crate::execution::run_tools(&commands, file_path, timeout, set_htmlbeautifier_args)
}

#[cfg(test)]
mod test_htmlbeautifier {
    #[test_with::executable(htmlbeautifier)]
    fn test_htmlbeautifier_html_6fdcd2ce3ffada76() {
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
</div>
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("html");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::htmlbeautifier::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
