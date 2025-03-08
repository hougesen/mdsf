#[inline]
pub fn setup_command_from_path(path: &str, binary_name: &str) -> std::process::Command {
    // TODO: logic to determine if binary in parent/sub folder
    let mut cmd = std::process::Command::new(format!("./{binary_name}"));

    // TODO: when running tests they are executed from the ./mdsf/ folder which means everything is out of sync with local node_modules
    cmd.current_dir(path);

    cmd
}
