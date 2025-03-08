#[inline]
pub fn setup_uv_run_command(package_name: &str) -> std::process::Command {
    let mut command = std::process::Command::new("uv");

    command.arg("tool");
    command.arg("run");
    command.arg("--no-progress");
    command.arg(package_name);

    command
}
