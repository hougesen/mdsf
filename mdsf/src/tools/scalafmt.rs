///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg("--mode");
    cmd.arg("any");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("scalafmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_scalafmt {
    #[test_with::executable(scalafmt)]
    fn test_scalafmt_scala_cbd61c065383c05b() {
        let input = r#"object Addition {
             def main() = {
                 println(1 + 3)
             }
    }"#;

        let output = r#"object Addition {
  def main() = {
    println(1 + 3)
  }
}
"#;

        let file_ext = crate::fttype::get_file_extension("scala");

        crate::tools::Tooling::Scalafmt.test_format_snippet(input, output, &file_ext);
    }
}
