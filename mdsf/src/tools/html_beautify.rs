///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_html_beautify_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("html");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("html-beautify"),
        CommandType::Direct("html-beautify"),
        CommandType::Npm("js-beautify"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_html_beautify_args)
}

#[cfg(test)]
mod test_html_beautify {
    #[test_with::executable(npx)]
    fn test_html_beautify_html_11e43869538b61e8() {
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
        let result = crate::tools::html_beautify::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
