///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--simple-exit");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] = [CommandType::Direct("rufo"), CommandType::GemExec("rufo")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_rufo {
    #[test_with::executable(rufo || gem)]
    fn test_rufo_ruby_d2b8a6db3c8eee1c() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("ruby");

        crate::tools::Tooling::Rufo.test_format_snippet(input, output, &file_ext);
    }
}
