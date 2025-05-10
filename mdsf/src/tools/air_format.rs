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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("air")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_air_format {
    #[test_with::executable(air)]
    fn test_air_format_r_b395a8aabbe68c56() {
        let input = r#"data            |>
                  select(foo)

  foo         <- function(bar         =                               1, baz=2)                                 {
   list(bar,                 baz)
 }

"#;

        let output = r#"data |>
  select(foo)

foo <- function(bar = 1, baz = 2) {
  list(bar, baz)
}
"#;

        let file_ext = crate::fttype::get_file_extension("r");

        crate::tools::Tooling::AirFormat.test_format_snippet(input, output, &file_ext);
    }
}
