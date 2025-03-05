///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(format!("'{}'", file_path.to_string_lossy()));
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("erlfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_erlfmt {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(erlfmt)]
    fn test_erlfmt_erlang_61f4ac26ad7484d2() {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end."#;

        let file_ext = crate::fttype::get_file_extension("erlang");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
