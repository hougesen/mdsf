use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_beautysh(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("beautysh");

    cmd.arg(file_path);

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_beautysh {
    use crate::{
        formatters::{beautysh::format_using_beautysh, setup_snippet},
        languages::{Language, ShellFlavor},
    };

    #[test_with::executable(beautysh)]
    #[test]
    fn it_should_format_sh() {
        let input = "#!/bin/shell

       add() {
    echo \"$1\" + \"$2\"
             }
";
        let expected_output = "#!/bin/shell

add() {
    echo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell(ShellFlavor::Shell).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_beautysh(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(beautysh)]
    #[test]
    fn it_should_format_bash() {
        let input = "#!/bin/bash

       add() {
    echo \"$1\" + \"$2\"
             }
";
        let expected_output = "#!/bin/bash

add() {
    echo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell(ShellFlavor::Bash).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_beautysh(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
