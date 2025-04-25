#[inline]
pub fn setup_dotnet_command(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("dotnet");

    cmd.arg(package_name);

    cmd
}
