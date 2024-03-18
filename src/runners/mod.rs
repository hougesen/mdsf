use schemars::JsonSchema;

use self::bun::new_bunx_cmd;
use self::deno::new_deno_cmd;
use self::node::new_npx_cmd;

mod bun;
mod deno;
mod node;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum JavaScriptRuntime {
    #[serde(rename = "bun")]
    Bun,
    #[serde(rename = "deno")]
    Deno,
    #[default]
    #[serde(rename = "node")]
    Node,
}

pub fn setup_npm_script(runner: JavaScriptRuntime, package_name: &str) -> std::process::Command {
    match runner {
        JavaScriptRuntime::Bun => new_bunx_cmd(package_name),
        JavaScriptRuntime::Deno => new_deno_cmd(package_name),
        JavaScriptRuntime::Node => new_npx_cmd(package_name),
    }
}
