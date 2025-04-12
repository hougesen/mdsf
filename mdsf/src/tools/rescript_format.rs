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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("rescript"),
    CommandType::Direct("rescript"),
    CommandType::Npm("rescript"),
    CommandType::Pnpm("rescript"),
    CommandType::Bun("rescript"),
    CommandType::Deno("rescript"),
    CommandType::Yarn("rescript"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_rescript_format {
    #[test_with::executable(rescript || npx || pnpm || deno || bunx)]
    fn test_rescript_format_rescript_59c7490e2a041de3() {
        let input = r#"module Button = {
  @react.component
  let make = (~count) =>   {
    let times = switch    count {
            | 1          =>   "once"
    | 2  =>         "twice"
    |   n =>      n->Int.toString ++ " times"
     }
     let text =                           `Click me ${times}`

    <button> {text->React.string} </button>
  }
}"#;

        let output = r#"module Button = {
  @react.component
  let make = (~count) => {
    let times = switch count {
    | 1 => "once"
    | 2 => "twice"
    | n => n->Int.toString ++ " times"
    }
    let text = `Click me ${times}`

    <button> {text->React.string} </button>
  }
}
"#;

        let file_ext = crate::fttype::get_file_extension("rescript");

        crate::tools::Tooling::RescriptFormat.test_format_snippet(input, output, &file_ext);
    }
}
