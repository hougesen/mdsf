use crate::{
    runners::{setup_npm_script, JavaScriptRuntime},
    terminal::print_formatter_info,
};

use super::execute_command;

#[inline]
fn set_rescript_format_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("format").arg(snippet_path);
}

#[inline]
fn invote_rescript_format(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    set_rescript_format_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_rescript_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("rescript_format");

    invote_rescript_format(
        setup_npm_script(JavaScriptRuntime::default(), "rescript"),
        snippet_path,
    )
}

#[cfg(test)]
mod test_rescript_format {
    use crate::{formatters::setup_snippet, languages::Language};

    #[test]
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

        let snippet = setup_snippet(input, Language::ReScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = super::format_using_rescript_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
