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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("efmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_efmt {
    #[test_with::executable(efmt)]
    fn test_efmt_erlang_d4d88e49805fdb39() {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"#;

        let file_ext = crate::fttype::get_file_extension("erlang");

        crate::tools::Tooling::Efmt.test_format_snippet(input, output, &file_ext);
    }
}
