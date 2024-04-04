use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_erlfmt(file_path: &std::path::Path) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("erlfmt");

    let mut cmd = std::process::Command::new("erlfmt");

    cmd.arg("-w")
        .arg(format!("'{}'", file_path.to_string_lossy()));

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_erlfmt {
    use crate::{
        formatters::{erlfmt::format_using_erlfmt, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(erlfmt)]
    #[test]
    fn it_should_format_erlang() {
        let input = "what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
.";

        let expected_output = "what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end.";
        let snippet = setup_snippet(input, Language::Erlang.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_erlfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
