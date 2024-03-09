use super::{execute_command, read_snippet};

#[inline]
pub fn format_using_shfmt(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("shfmt");

    // Incase the use hasn't installed biome
    cmd.arg("--write").arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_shfmt {
    use crate::{
        formatters::{setup_snippet, shfmt::format_using_shfmt},
        languages::Language,
    };

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

        let snippet = setup_snippet(input, Language::Shell.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_shfmt(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_bash() {
        let input = "

#!/bin/shell

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";
        let expected_output = "#!/bin/shell

add() {
\techo \"$1\" + \"$2\"
}
";

        let snippet = setup_snippet(input, Language::Shell.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_shfmt(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
