#[inline]
pub fn new_pnpm_dlx_cmd(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("pnpm");

    cmd.arg("dlx").arg(package_name);

    cmd
}
