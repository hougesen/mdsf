mod bun;
mod composer;
mod deno;
mod dotnet;
mod dub;
mod gem;
mod node;
mod path;
mod pipx;
mod pnpm;
mod uv;
mod yarn;

#[derive(Debug)]
pub enum CommandType {
    BinaryPath(&'static str, &'static str),
    Bun(&'static str, &'static str),
    Deno(&'static str, &'static str),
    Direct(&'static str),
    Dotnet(&'static str),
    Dub(&'static str),
    GemExec(&'static str),
    NodeModules(&'static str),
    Npm(&'static str, &'static str),
    PhpVendor(&'static str),
    Pipx(&'static str, &'static str),
    Pnpm(&'static str, &'static str),
    Uv(&'static str, &'static str),
    Yarn(&'static str, &'static str),
}

impl CommandType {
    #[inline]
    pub const fn is_enabled(&self, config_runners: &crate::config::MdsfConfigRunners) -> bool {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::BinaryPath(_, _) => true,
            Self::Bun(_, _) => config_runners.bunx,
            Self::Deno(_, _) => config_runners.deno,
            Self::Direct(_) => true,
            Self::Dotnet(_) => config_runners.dotnet,
            Self::Dub(_) => config_runners.dub,
            Self::GemExec(_) => config_runners.gem_exec,
            Self::NodeModules(_) => true,
            Self::Npm(_, _) => config_runners.npx,
            Self::PhpVendor(_) => true,
            Self::Pipx(_, _) => config_runners.pipx,
            Self::Pnpm(_, _) => config_runners.pnpm,
            Self::Uv(_, _) => config_runners.uv,
            Self::Yarn(_, _) => config_runners.yarn,
        }
    }

    #[inline]
    pub fn build(&self) -> std::process::Command {
        match self {
            Self::BinaryPath(path, binary_name) => path::setup_command_from_path(path, binary_name),
            Self::Bun(package_name, executable_name) => {
                bun::setup_bunx_command(package_name, executable_name)
            }
            Self::Deno(package_name, executable_name) => {
                deno::setup_deno_run_command(package_name, executable_name)
            }
            Self::Direct(binary_name) => std::process::Command::new(binary_name),
            Self::Dotnet(package_name) => dotnet::setup_dotnet_command(package_name),
            Self::Dub(package_name) => dub::setup_dub_run_command(package_name),
            Self::GemExec(package_name) => gem::setup_gem_exec_command(package_name),
            Self::NodeModules(binary_name) => node::setup_node_modules_command(binary_name),
            Self::Npm(package_name, executable_name) => {
                node::setup_npx_command(package_name, executable_name)
            }
            Self::PhpVendor(binary_name) => composer::setup_php_vender_bin_command(binary_name),
            Self::Pipx(package_name, executable_name) => {
                pipx::setup_pipx_run_command(package_name, executable_name)
            }
            Self::Pnpm(package_name, executable_name) => {
                pnpm::setup_pnpm_dlx_command(package_name, executable_name)
            }
            Self::Uv(package_name, executable_name) => {
                uv::setup_uv_run_command(package_name, executable_name)
            }
            Self::Yarn(package_name, executable_name) => {
                yarn::setup_yarn_dlx_command(package_name, executable_name)
            }
        }
    }
}
