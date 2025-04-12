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

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mix_format {
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

        crate::tools::Tooling::MixFormat.test_format_snippet(input, output, &file_ext);
    }
}
