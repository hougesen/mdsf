use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("beautysh").build();

    cmd.arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_beautysh {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(beautysh)]
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

        let snippet = setup_snippet(input, &get_file_extension("shell"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(beautysh)]
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

        let snippet =
            setup_snippet(input, &get_file_extension("bash")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
