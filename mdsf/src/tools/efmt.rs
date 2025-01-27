///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_efmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("efmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_efmt_args)
}

#[cfg(test)]
mod test_efmt {
    #[test_with::executable(efmt)]
    fn test_efmt_erlang_178ba575237af84d() {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;
        let output = Some(
            r#"what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("erlang");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::efmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
