#[inline]
pub fn new_bunx_cmd(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("bunx");

    // Auto install package
    cmd.arg("--yes");

    cmd.arg(package_name);

    cmd
}
