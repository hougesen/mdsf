use bun::new_bunx_cmd;
use deno::new_deno_cmd;
use node::new_npx_cmd;
use pnpm::new_pnpm_dlx_cmd;

mod bun;
mod deno;
mod node;
mod pnpm;

#[derive(
    Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy, Hash, PartialEq, Eq,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum JavaScriptRuntime {
    #[serde(rename = "bun")]
    Bun,
    #[serde(rename = "deno")]
    Deno,
    #[default]
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "pnpm")]
    Pnpm,
}

impl JavaScriptRuntime {
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl core::fmt::Display for JavaScriptRuntime {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bun => f.write_str("bun"),
            Self::Deno => f.write_str("deno"),
            Self::Node => f.write_str("node"),
            Self::Pnpm => f.write_str("pnpm"),
        }
    }
}

#[inline]
fn setup_npm_script(runtime: JavaScriptRuntime, package_name: &str) -> std::process::Command {
    match runtime {
        JavaScriptRuntime::Bun => new_bunx_cmd(package_name),
        JavaScriptRuntime::Deno => new_deno_cmd(package_name),
        JavaScriptRuntime::Node => new_npx_cmd(package_name),
        JavaScriptRuntime::Pnpm => new_pnpm_dlx_cmd(package_name),
    }
}

#[inline]
fn setup_command_from_path(path: &str, binary_name: &str) -> std::process::Command {
    // TODO: logic to determine if binary in parent/sub folder
    let mut cmd = std::process::Command::new(format!("./{binary_name}"));

    cmd.current_dir(path);

    cmd
}

#[inline]
fn setup_node_modules_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./node_modules/.bin/", binary_name)
}

#[inline]
fn setup_php_vender_bin_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./vendor/bin/", binary_name)
}

#[derive(Debug)]
pub enum CommandType {
    BinaryPath(&'static str, &'static str),
    Direct(&'static str),
    NodeModules(&'static str),
    Npm(&'static str),
    PhpVendor(&'static str),
}

impl CommandType {
    #[inline]
    pub fn build(&self, javascript_runtime: JavaScriptRuntime) -> std::process::Command {
        match self {
            Self::BinaryPath(path, binary_name) => setup_command_from_path(path, binary_name),
            Self::Direct(binary_name) => std::process::Command::new(binary_name),
            Self::NodeModules(binary_name) => setup_node_modules_command(binary_name),
            Self::Npm(package_name) => setup_npm_script(javascript_runtime, package_name),
            Self::PhpVendor(binary_name) => setup_php_vender_bin_command(binary_name),
        }
    }
}
