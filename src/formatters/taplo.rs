use super::{execute_command, read_snippet};

pub fn format_using_taplo(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("taplo");

    cmd.arg("fmt");
    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_taplo {
    use crate::{
        formatters::{setup_snippet, taplo::format_using_taplo},
        languages::Language,
    };

    #[test]
    fn it_should_format_toml() {
        let input = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\" 
  ";

        let expected_output = "package = \"mdsf\"
author = \"Mads Hougesen\"
";

        let snippet = setup_snippet(input, Language::Toml.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_taplo(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
