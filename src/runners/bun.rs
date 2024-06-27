#[inline]
pub fn new_bunx_cmd(package_name: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("bunx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}
