use mdsf::{config::MdsfConfigRunners, execution::setup_snippet, tools::Tooling};

pub const DEFAULT_TEST_FORMATTER_TIMEOUT: u64 = 0;

pub const DEFAULT_TEST_DEBUG_ENABLED: bool = true;

pub fn run_tooling_test(tool: Tooling, input: &str, output: &str, file_ext: &str) {
    let snippet = setup_snippet(input, file_ext).expect("it to create a snippet file");

    let result = tool
        .format_snippet(
            snippet.path(),
            DEFAULT_TEST_FORMATTER_TIMEOUT,
            DEFAULT_TEST_DEBUG_ENABLED,
            &MdsfConfigRunners::all(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

    assert_eq!(result, output);
}
