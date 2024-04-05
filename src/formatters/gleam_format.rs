use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_gleam_format(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("gleam");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_gleam_format {
    use crate::{
        formatters::{gleam_format::format_using_gleam_format, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(gleam)]
    #[test]
    fn it_should_format_gleam() {
        let input = "pub fn add(a:Int,b:Int)->Int{a+b}";
        let expected_output = "pub fn add(a: Int, b: Int) -> Int {
  a + b
}
";

        let snippet = setup_snippet(input, Language::Gleam.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_gleam_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
