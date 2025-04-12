///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("rubyfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_rubyfmt {
    #[test_with::executable(rubyfmt)]
    fn test_rubyfmt_ruby_d2b8a6db3c8eee1c() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("ruby");

        crate::tools::Tooling::Rubyfmt.test_format_snippet(input, output, &file_ext);
    }
}
