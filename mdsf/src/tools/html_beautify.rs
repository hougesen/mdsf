use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

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
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("html-beautify"),
        CommandType::Direct("html-beautify"),
        CommandType::Npm("js-beautify"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_html_beautify_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

#[cfg(test)]
mod test_html_beautify {
    #[test_with::executable(npx)]
    fn test_html_beautify_html_63850f31f2ef5caf() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;
        let output = r#"<div>
    <p>
        Mads was here
    </p>
</div>"#;
        let file_ext = crate::fttype::get_file_extension("html");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::html_beautify::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
