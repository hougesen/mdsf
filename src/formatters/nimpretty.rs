use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("nimpretty");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_nimpretty {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(nimpretty)]
    fn it_should_format_nim() {
        let input = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

        let expected_output = "proc add(a: int, b: int): int =
  return a + b
";

        let snippet =
            setup_snippet(input, language_to_ext("nim")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
