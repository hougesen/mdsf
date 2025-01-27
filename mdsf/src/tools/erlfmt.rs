///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_erlfmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(format!("'{}'", file_path.to_string_lossy()));
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("erlfmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_erlfmt_args)
}

#[cfg(test)]
mod test_erlfmt {
    #[test_with::executable(erlfmt)]
    fn test_erlfmt_erlang_90fa4b828bc9873() {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;
        let output = Some(
            r#"what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end."#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("erlang");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::erlfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
