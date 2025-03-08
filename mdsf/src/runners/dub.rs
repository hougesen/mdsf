#[inline]
pub fn setup_dub_run_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("dub");

    cmd.arg("run");
    cmd.arg(package_name);
    cmd.arg("--");

    cmd
}
