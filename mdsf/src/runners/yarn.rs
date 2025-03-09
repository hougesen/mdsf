#[inline]
pub fn setup_yarn_dlx_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("yarn");

    cmd.arg("dlx");
    cmd.arg("--quiet");
    cmd.arg(package_name);

    cmd
}
