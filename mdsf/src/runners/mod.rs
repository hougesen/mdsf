use crate::config::MdsfConfigRunners;

mod bun;
mod composer;
mod deno;
mod node;
mod pipx;
mod pnpm;
mod uv;

#[inline]
fn setup_command_from_path(path: &str, binary_name: &str) -> std::process::Command {
    // TODO: logic to determine if binary in parent/sub folder
    let mut cmd = std::process::Command::new(format!("./{binary_name}"));

    // TODO: when running tests they are executed from the ./mdsf/ folder which means everything is out of sync with local node_modules
    cmd.current_dir(path);

    cmd
}

#[inline]
fn setup_node_modules_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./node_modules/.bin/", binary_name)
}

#[derive(Debug)]
pub enum CommandType {
    BinaryPath(&'static str, &'static str),
    Bun(&'static str),
    Deno(&'static str),
    Direct(&'static str),
    NodeModules(&'static str),
    Npm(&'static str),
    PhpVendor(&'static str),
    PipxRun(&'static str),
    Pnpm(&'static str),
    Uv(&'static str),
}

impl CommandType {
    #[inline]
    pub fn is_enabled(&self, config_runners: &MdsfConfigRunners) -> bool {
        match self {
            CommandType::BinaryPath(_, _) => true,
            CommandType::Direct(_) => true,
            CommandType::NodeModules(_) => true,
            CommandType::PhpVendor(_) => true,
            CommandType::PipxRun(_) => config_runners
                .python
                .as_ref()
                .is_some_and(|python| python.pipx),
            CommandType::Uv(_) => config_runners
                .python
                .as_ref()
                .is_some_and(|python| python.uv),
            CommandType::Deno(_) => config_runners
                .javascript
                .as_ref()
                .is_some_and(|javascript| javascript.deno),
            CommandType::Bun(_) => config_runners
                .javascript
                .as_ref()
                .is_some_and(|javascript| javascript.bunx),
            CommandType::Npm(_) => config_runners
                .javascript
                .as_ref()
                .is_some_and(|javascript| javascript.npx),
            CommandType::Pnpm(_) => config_runners
                .javascript
                .as_ref()
                .is_some_and(|javascript| javascript.pnpm),
        }
    }
}

impl CommandType {
    #[inline]
    pub fn build(&self) -> std::process::Command {
        match self {
            Self::BinaryPath(path, binary_name) => setup_command_from_path(path, binary_name),
            Self::Bun(package_name) => bun::setup_bunx_command(package_name),
            Self::Deno(package_name) => deno::setup_deno_run_command(package_name),
            Self::Direct(binary_name) => std::process::Command::new(binary_name),
            Self::NodeModules(binary_name) => setup_node_modules_command(binary_name),
            Self::Npm(package_name) => node::setup_npx_command(package_name),
            Self::PhpVendor(binary_name) => composer::setup_php_vender_bin_command(binary_name),
            Self::PipxRun(package_name) => pipx::setup_command(package_name),
            Self::Pnpm(package_name) => pnpm::setup_pnpm_dlx_command(package_name),
            Self::Uv(package_name) => uv::setup_command(package_name),
        }
    }
}
