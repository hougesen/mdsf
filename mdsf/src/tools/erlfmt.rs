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

        crate::tools::Tooling::Erlfmt.test_format_snippet(input, output, &file_ext);
    }
}
