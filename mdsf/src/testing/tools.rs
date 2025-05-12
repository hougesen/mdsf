#[cfg(test)]
impl crate::tools::Tooling {
    #[cfg(test)]
    pub fn test_format_snippet(self, input: &str, output: &str, file_ext: &str) {
        let snippet =
            crate::execution::setup_snippet(input, file_ext).expect("it to create a snippet file");

        let result = self
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
