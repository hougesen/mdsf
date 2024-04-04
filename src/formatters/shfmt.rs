use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_shfmt(file_path: &std::path::Path) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("shfmt");

    let mut cmd = std::process::Command::new("shfmt");

    cmd.arg("--write").arg(file_path);

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_shfmt {
    use crate::{
        formatters::{setup_snippet, shfmt::format_using_shfmt},
        languages::{Language, ShellFlavor},
    };

    #[test_with::executable(shfmt)]
    #[test]
    fn it_should_format_sh() {
        let input = "

#!/bin/sh

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";
        let expected_output = "#!/bin/sh

add() {
\techo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell(ShellFlavor::Shell).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_shfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(shfmt)]
    #[test]
    fn it_should_format_bash() {
        let input = "

#!/bin/bash

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";
        let expected_output = "#!/bin/bash

add() {
\techo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell(ShellFlavor::Bash).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_shfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(shfmt)]
    #[test]
    fn it_should_format_zsh() {
        let input = "

#!/bin/zsh

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";
        let expected_output = "#!/bin/zsh

add() {
\techo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell(ShellFlavor::Zsh).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_shfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
