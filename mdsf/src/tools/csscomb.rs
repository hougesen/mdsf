///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-t");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("csscomb"),
    CommandType::Direct("csscomb"),
    CommandType::Npm("csscomb"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_csscomb {
    #[test_with::executable(npx)]
    fn test_csscomb_css_bed67a883a4a1aae() {
        let input = r#"h1   {color: blue;}
p {color: red;}"#;

        let output = r#"h1
{
    color: blue;
}
p
{
    color: red;
}
"#;

        let file_ext = crate::fttype::get_file_extension("css");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Csscomb
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
