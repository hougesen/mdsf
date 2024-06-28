use super::execute_command;
use crate::{
    error::MdsfError,
    runners::{run_executable_from_path, setup_npm_script},
};

#[inline]
fn set_rescript_format_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_rescript_format(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_rescript_format_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_rescript_format(
        run_executable_from_path("node_modules/.bin/rescript"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_rescript_format(std::process::Command::new("rescript"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_rescript_format(setup_npm_script("rescript"), snippet_path)
}

#[cfg(test)]
mod test_rescript_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_rescript() {
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

        let expected_output = r#"module Button = {
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

        let snippet =
            setup_snippet(input, language_to_ext("rescript")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
