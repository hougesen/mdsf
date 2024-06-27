use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("beautysh");

    cmd.arg(file_path);

    execute_command(&mut cmd, file_path).await
}

#[cfg(test)]
mod test_beautysh {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(beautysh)]
    async fn it_should_format_sh() {
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

        let snippet = setup_snippet(input, language_to_ext("shell"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[tokio::test]
    #[test_with::executable(beautysh)]
    async fn it_should_format_bash() {
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

        let snippet = setup_snippet(input, language_to_ext("bash"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
