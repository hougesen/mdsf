use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("efmt");

    cmd.arg("-w").arg(file_path);

    execute_command(&mut cmd, file_path)
}

#[cfg(test)]
mod test_efmt {
    use crate::{
        formatters::{efmt::run, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(efmt)]
    fn it_should_format_erlang() {
        let input = "what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
.";

        let expected_output = "what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"
;
        let snippet =
            setup_snippet(input, language_to_ext("erlang")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
