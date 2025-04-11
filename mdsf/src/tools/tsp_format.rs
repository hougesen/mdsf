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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("tsp")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_tsp_format {
    #[test_with::executable(tsp)]
    fn test_tsp_format_typespec_f4c58025c5f05edc() {
        let input = r#"model Pet {  name: string;  age: int32;kind: "dog" | "cat" | "fish";}
"#;

        let output = r#"model Pet {
  name: string;
  age: int32;
  kind: "dog" | "cat" | "fish";
}
"#;

        let file_ext = crate::fttype::get_file_extension("typespec");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::TspFormat
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
