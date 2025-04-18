///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("deno")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_deno_fmt {
    #[test_with::executable(deno)]
    fn test_deno_fmt_javascript_d7445fa122fcd5cc() {
        let input = r#"
    async function asyncAddition(a,b){
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::DenoFmt.test_format_snippet(input, output, &file_ext);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_typescript_857476c85438ce71() {
        let input = r#"
    async function asyncAddition(                                a:       	number,b:number ) :Promise< number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("typescript");

        crate::tools::Tooling::DenoFmt.test_format_snippet(input, output, &file_ext);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_json_d426a9ade74002d2() {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        crate::tools::Tooling::DenoFmt.test_format_snippet(input, output, &file_ext);
    }
}
