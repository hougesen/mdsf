use crate::config::MdsfConfigRunners;

mod bun;
mod composer;
mod deno;
mod dub;
mod node;
mod path;
mod pipx;
mod pnpm;
mod uv;

#[derive(Debug)]
pub enum CommandType {
    BinaryPath(&'static str, &'static str),
    Bun(&'static str),
    Deno(&'static str),
    Direct(&'static str),
    Dub(&'static str),
    NodeModules(&'static str),
    Npm(&'static str),
    PhpVendor(&'static str),
    Pipx(&'static str),
    Pnpm(&'static str),
    Uv(&'static str),
}

impl CommandType {
    #[inline]
    pub fn is_enabled(&self, config_runners: &MdsfConfigRunners) -> bool {
        match self {
            Self::BinaryPath(_, _) => true,
            Self::Direct(_) => true,
            Self::Dub(_) => config_runners.dub,
            Self::NodeModules(_) => true,
            Self::PhpVendor(_) => true,
            Self::Pipx(_) => config_runners
                .pypi
                .as_ref()
                .is_some_and(|python| python.pipx),
            Self::Uv(_) => config_runners.pypi.as_ref().is_some_and(|python| python.uv),
            Self::Deno(_) => config_runners
                .npm
                .as_ref()
                .is_some_and(|javascript| javascript.deno),
            Self::Bun(_) => config_runners
                .npm
                .as_ref()
                .is_some_and(|javascript| javascript.bunx),
            Self::Npm(_) => config_runners
                .npm
                .as_ref()
                .is_some_and(|javascript| javascript.npx),
            Self::Pnpm(_) => config_runners
                .npm
                .as_ref()
                .is_some_and(|javascript| javascript.pnpm),
        }
    }

    #[inline]
    pub fn build(&self) -> std::process::Command {
        match self {
            Self::BinaryPath(path, binary_name) => path::setup_command_from_path(path, binary_name),
            Self::Bun(package_name) => bun::setup_bunx_command(package_name),
            Self::Deno(package_name) => deno::setup_deno_run_command(package_name),
            Self::Direct(binary_name) => std::process::Command::new(binary_name),
            Self::Dub(package_name) => dub::setup_dub_run_command(package_name),
            Self::NodeModules(binary_name) => node::setup_node_modules_command(binary_name),
            Self::Npm(package_name) => node::setup_npx_command(package_name),
            Self::PhpVendor(binary_name) => composer::setup_php_vender_bin_command(binary_name),
            Self::Pipx(package_name) => pipx::setup_pipx_run_command(package_name),
            Self::Pnpm(package_name) => pnpm::setup_pnpm_dlx_command(package_name),
            Self::Uv(package_name) => uv::setup_uv_run_command(package_name),
        }
    }
}
