use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("erlfmt");

    cmd.arg("-w")
        .arg(format!("'{}'", file_path.to_string_lossy()));

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_erlfmt {
    use crate::{
        formatters::{erlfmt::run, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(erlfmt)]
    fn it_should_format_erlang() {
        let input = "what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
.";

        let expected_output = "what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end.";
        let snippet =
            setup_snippet(input, language_to_ext("erlang")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
