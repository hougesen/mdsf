#[derive(serde::Serialize, serde::Deserialize, Hash, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct CustomTool {
    pub binary: String,

    pub arguments: Vec<String>,

    #[serde[default]]
    pub stdin: bool,
}

impl core::fmt::Display for CustomTool {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (custom)", self.binary)
    }
}

#[inline]
fn build_set_args_fn(
    arguments: Vec<String>,
) -> impl Fn(std::process::Command, &std::path::Path) -> std::process::Command {
    move |mut cmd: std::process::Command, file_path: &std::path::Path| -> std::process::Command {
        for arg in &arguments {
            if arg == "$PATH" {
                cmd.arg(file_path);
            } else if arg.contains("$PATH_STRING") {
                cmd.arg(arg.replace(
                    "$PATH_STRING",
                    &format!("\"{}\"", file_path.to_string_lossy()),
                ));
            } else if arg.contains("$PATH") {
                cmd.arg(arg.replace("$PATH", &file_path.to_string_lossy()));
            }
            {
                cmd.arg(arg);
            }
        }

        cmd
    }
}

impl CustomTool {
    #[inline]
    pub fn tool_name(&self) -> &str {
        &self.binary
    }

    #[inline]
    pub fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
        timeout: u64,
        debug_enabled: bool,
        config_runners: &crate::config::MdsfConfigRunners,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        let command = crate::runners::CommandType::Custom(self.binary.clone());

        let is_stdin = self.stdin;

        let args_fn = build_set_args_fn(self.arguments.clone());

        crate::execution::run_tools(
            &[command],
            snippet_path,
            args_fn,
            timeout,
            is_stdin,
            debug_enabled,
            config_runners,
        )
    }
}

#[cfg(test)]
mod test_custom_tool {
    use crate::{
        config::{MdsfConfig, MdsfConfigRunners, MdsfTool},
        custom::CustomTool,
        execution::MdsfToolWrapper,
        format_file,
        testing::{
            DEFAULT_ON_MISSING_LANGUAGE_DEFINITION, DEFAULT_ON_MISSING_TOOL_BINARY,
            DEFAULT_TEST_DEBUG_ENABLED, DEFAULT_TEST_FORMATTER_TIMEOUT,
        },
    };

    #[test]
    fn with_stdin_false() {
        let input = r#"```rust
pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}

```"#;

        let expected_output = r#"```rust
pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
"#;

        let tool = CustomTool {
            binary: "rustfmt".to_string(),
            arguments: vec![
                "--edition".to_owned(),
                "2024".to_owned(),
                "--quiet".to_owned(),
                "$PATH".to_owned(),
            ],
            stdin: false,
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfToolWrapper::Single(MdsfTool::Custom(tool.clone())),
                )]),
                runners: MdsfConfigRunners::default(),
                ..Default::default()
            };

            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                DEFAULT_TEST_FORMATTER_TIMEOUT,
                DEFAULT_TEST_DEBUG_ENABLED,
                DEFAULT_ON_MISSING_TOOL_BINARY,
                DEFAULT_ON_MISSING_LANGUAGE_DEFINITION,
            );

            assert!(modified);

            assert_eq!(output, expected_output);
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "rust".to_string(),
                    MdsfToolWrapper::Multiple(vec![MdsfToolWrapper::Single(MdsfTool::Custom(
                        tool,
                    ))]),
                )]),
                runners: MdsfConfigRunners::default(),
                ..Default::default()
            };

            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                DEFAULT_TEST_FORMATTER_TIMEOUT,
                DEFAULT_TEST_DEBUG_ENABLED,
                DEFAULT_ON_MISSING_TOOL_BINARY,
                DEFAULT_ON_MISSING_LANGUAGE_DEFINITION,
            );

            assert!(modified);

            assert_eq!(output, expected_output);
        };
    }

    #[test]
    fn with_stdin_true() {
        let input = r#"```toml
[project ]
name =     "hello"
```"#;

        let expected_output = r#"```toml
[project]
name = "hello"
```
"#;

        let tool = CustomTool {
            binary: "tombi".to_string(),
            arguments: vec!["format".to_owned(), "-".to_owned()],
            stdin: true,
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "toml".to_string(),
                    MdsfToolWrapper::Single(MdsfTool::Custom(tool.clone())),
                )]),
                runners: MdsfConfigRunners::default(),
                ..Default::default()
            };

            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                DEFAULT_TEST_FORMATTER_TIMEOUT,
                DEFAULT_TEST_DEBUG_ENABLED,
                DEFAULT_ON_MISSING_TOOL_BINARY,
                DEFAULT_ON_MISSING_LANGUAGE_DEFINITION,
            );

            assert!(modified);

            assert_eq!(output, expected_output);
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "toml".to_string(),
                    MdsfToolWrapper::Multiple(vec![MdsfToolWrapper::Single(MdsfTool::Custom(
                        tool,
                    ))]),
                )]),
                runners: MdsfConfigRunners::default(),
                ..Default::default()
            };

            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                DEFAULT_TEST_FORMATTER_TIMEOUT,
                DEFAULT_TEST_DEBUG_ENABLED,
                DEFAULT_ON_MISSING_TOOL_BINARY,
                DEFAULT_ON_MISSING_LANGUAGE_DEFINITION,
            );

            assert!(modified);

            assert_eq!(output, expected_output);
        };
    }
}
