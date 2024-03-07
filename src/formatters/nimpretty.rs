use super::{execute_command, read_snippet};

pub fn format_using_nimpretty(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("nimpretty");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_nimpretty {
    use crate::{
        formatters::{nimpretty::format_using_nimpretty, setup_snippet},
        languages::Language,
    };

    #[test]
    fn it_should_format_toml() {
        let input = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

        let expected_output = "proc add(a: int, b: int): int =
  return a + b
";

        let snippet =
            setup_snippet(input, Language::Nim.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_nimpretty(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
