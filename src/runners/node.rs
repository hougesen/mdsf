#[inline]
pub fn new_npx_cmd(package_name: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("npx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}
