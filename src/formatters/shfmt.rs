use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("shfmt");

    cmd.arg("--write").arg(file_path);

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_shfmt {
    use crate::{
        formatters::{setup_snippet, shfmt::run},
        generated::language_to_ext,
    };

    #[test_with::executable(shfmt)]
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

        let snippet =
            setup_snippet(input, &language_to_ext("shell")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(shfmt)]
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

        let snippet =
            setup_snippet(input, &language_to_ext("bash")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(shfmt)]
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

        let snippet =
            setup_snippet(input, &language_to_ext("zsh")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
