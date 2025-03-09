mod bun;
mod composer;
mod deno;
mod dub;
mod node;
mod path;
mod pipx;
mod pnpm;
mod uv;
mod yarn;

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
    Yarn(&'static str),
}

impl CommandType {
    #[inline]
    pub const fn is_enabled(&self, config_runners: &crate::config::MdsfConfigRunners) -> bool {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::BinaryPath(_, _) => true,
            Self::Bun(_) => config_runners.bunx,
            Self::Deno(_) => config_runners.deno,
            Self::Direct(_) => true,
            Self::Dub(_) => config_runners.dub,
            Self::NodeModules(_) => true,
            Self::Npm(_) => config_runners.npx,
            Self::PhpVendor(_) => true,
            Self::Pipx(_) => config_runners.pipx,
            Self::Pnpm(_) => config_runners.pnpm,
            Self::Uv(_) => config_runners.uv,
            Self::Yarn(_) => config_runners.yarn,
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
            Self::Yarn(package_name) => yarn::setup_yarn_dlx_command(package_name),
        }
    }
}
