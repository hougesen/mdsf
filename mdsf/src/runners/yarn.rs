#[inline]
pub fn setup_yarn_dlx_command(package_name: &str, executable_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("yarn");

    cmd.arg("dlx");
    cmd.arg("--quiet");
    cmd.arg(package_name);

    cmd
}

#[cfg(test)]
mod test_yarn {
    #[test_with::executable(yarn)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        todo!()
    }

    #[test_with::executable(yarn)]
    #[test]
    fn it_works_with_executable_name() {
        todo!()
    }
}
