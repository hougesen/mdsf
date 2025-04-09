///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("snakefmt"),
    CommandType::Uv("snakefmt", "snakefmt"),
    CommandType::Pipx("snakefmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_snakefmt {
    #[test_with::executable(snakefmt || pipx || uv)]
    fn test_snakefmt_snakemake_cdccd086422a6b0a() {
        let input = r#"from snakemake.utils import min_version
min_version("5.14.0")
configfile: "config.yaml"
include: "rules/foo.smk"
"#;

        let output = r#"from snakemake.utils import min_version

min_version("5.14.0")


configfile: "config.yaml"


include: "rules/foo.smk"
"#;

        let file_ext = crate::fttype::get_file_extension("snakemake");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Snakefmt
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
