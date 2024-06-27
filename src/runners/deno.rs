#[inline]
pub fn new_deno_cmd(package_name: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("deno");

    cmd.arg("run").arg("-A").arg(format!("npm:{package_name}"));

    cmd
}
