///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("mix")];

#[cfg(test)]
mod test_mix_format {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(mix)]
    fn test_mix_format_elixir_ab535c627dfb140() {
        let input = r#"
        def              add(a  ,      b   )   do    a   +   b                 end

"#;

        let output = r#"def add(a, b) do
  a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("elixir");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
