use core::sync::atomic::{AtomicU8, Ordering};

use bun::new_bunx_cmd;
use deno::new_deno_cmd;
use node::new_npx_cmd;
use pnpm::new_pnpm_dlx_cmd;

use crate::terminal::print_unknown_javascript_runtime;

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
        *self == JavaScriptRuntime::default()
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

static RUNTIME_FLAG: AtomicU8 = AtomicU8::new(0);

const JAVASCRIPT_RUNTIME_NODE: u8 = 0;
const JAVASCRIPT_RUNTIME_BUN: u8 = 1;
const JAVASCRIPT_RUNTIME_DENO: u8 = 2;
const JAVASCRIPT_RUNTIME_PNPM: u8 = 3;

impl From<JavaScriptRuntime> for u8 {
    #[inline]
    fn from(value: JavaScriptRuntime) -> Self {
        match value {
            JavaScriptRuntime::Bun => JAVASCRIPT_RUNTIME_BUN,
            JavaScriptRuntime::Deno => JAVASCRIPT_RUNTIME_DENO,
            JavaScriptRuntime::Node => JAVASCRIPT_RUNTIME_NODE,
            JavaScriptRuntime::Pnpm => JAVASCRIPT_RUNTIME_PNPM,
        }
    }
}

impl TryFrom<u8> for JavaScriptRuntime {
    type Error = ();

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            JAVASCRIPT_RUNTIME_BUN => Ok(Self::Bun),
            JAVASCRIPT_RUNTIME_DENO => Ok(Self::Deno),
            JAVASCRIPT_RUNTIME_NODE => Ok(Self::Node),
            JAVASCRIPT_RUNTIME_PNPM => Ok(Self::Pnpm),
            _ => Err(()),
        }
    }
}

#[inline]
pub fn set_javascript_runtime(runtime: JavaScriptRuntime) {
    RUNTIME_FLAG.store(runtime.into(), Ordering::Relaxed);
}

#[inline]
fn get_javascript_runtime() -> JavaScriptRuntime {
    let value = RUNTIME_FLAG.load(Ordering::Relaxed);

    JavaScriptRuntime::try_from(value).unwrap_or_else(|()| {
        let fallback = JavaScriptRuntime::default();
        print_unknown_javascript_runtime(value, fallback);
        fallback
    })
}

#[inline]
fn setup_npm_script(package_name: &str) -> std::process::Command {
    let runtime = get_javascript_runtime();

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
    pub fn build(&self) -> std::process::Command {
        match self {
            Self::BinaryPath(path, binary_name) => setup_command_from_path(path, binary_name),
            Self::Direct(binary_name) => std::process::Command::new(binary_name),
            Self::NodeModules(binary_name) => setup_node_modules_command(binary_name),
            Self::Npm(package_name) => setup_npm_script(package_name),
            Self::PhpVendor(binary_name) => setup_php_vender_bin_command(binary_name),
        }
    }
}
