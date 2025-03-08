use super::setup_command_from_path;

#[inline]
pub fn setup_php_vender_bin_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./vendor/bin/", binary_name)
}
