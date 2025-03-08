///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(cmd: std::process::Command, _file_path: &std::path::Path) -> std::process::Command {
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("jqfmt")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_jqfmt {
    #[test_with::executable(jqfmt)]
    fn test_jqfmt_jq_634e34d16cece292() {
        let input = r#"{one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine)))}"#;

        let output = r#"{ one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine))) }"#;

        let file_ext = crate::fttype::get_file_extension("jq");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Jqfmt
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
