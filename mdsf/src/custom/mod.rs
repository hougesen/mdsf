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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (custom)", self.binary)
    }
}

fn build_set_args_fn(
    arguments: Vec<String>,
) -> impl Fn(std::process::Command, &std::path::Path) -> std::process::Command {
    move |mut cmd: std::process::Command, file_path: &std::path::Path| -> std::process::Command {
        for arg in &arguments {
            if arg == "$PATH" {
                cmd.arg(file_path);
            } else {
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
        let command = crate::runners::CommandType::Custom(self.binary.to_string());

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
