#[inline]
pub fn new_deno_cmd(package_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("deno");

    cmd.arg("run").arg("-A").arg(format!("npm:{package_name}"));

    cmd
}
