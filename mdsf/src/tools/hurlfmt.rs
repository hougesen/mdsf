///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("hurlfmt")];

#[cfg(test)]
mod test_hurlfmt {
    #[test_with::executable(hurlfmt)]
    fn test_hurlfmt_hurl_854a543be0e12a7f() {
        let input = r#"  GET        https://example.ord/cats/123           "#;

        let output = r#"GET https://example.ord/cats/123
"#;

        let file_ext = crate::fttype::get_file_extension("hurl");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
