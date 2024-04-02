use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_mix_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("mix_format");

    let mut cmd = std::process::Command::new("mix");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_mix_format {
    use crate::{
        formatters::{mix_format::format_using_mix_format, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(mix)]
    #[test]
    fn it_should_format_elixir() {
        let input = "
        def              add(a  ,      b   )   do    a   +   b                 end

";
        let expected_output = "def add(a, b) do
  a + b
end
";

        let snippet = setup_snippet(input, Language::Elixir.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_mix_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
