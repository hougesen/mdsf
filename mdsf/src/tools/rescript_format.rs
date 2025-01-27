///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_rescript_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("rescript"),
        CommandType::Direct("rescript"),
        CommandType::Npm("rescript"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_rescript_format_args)
}

#[cfg(test)]
mod test_rescript_format {
    #[test_with::executable(npx)]
    fn test_rescript_format_rescript_30add3bd800dbeaa() {
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
        let output = Some(
            r#"module Button = {
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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("rescript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::rescript_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
