use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("mix");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_mix_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(mix)]
    fn it_should_format_elixir() {
        let input = "
        def              add(a  ,      b   )   do    a   +   b                 end

";
        let expected_output = "def add(a, b) do
  a + b
end
";

        let snippet =
            setup_snippet(input, language_to_ext("elixir")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
