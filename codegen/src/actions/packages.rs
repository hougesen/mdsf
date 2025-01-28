use super::workflow::WorkflowJobsStep;
use crate::tools::{Tool, ToolPackagesBrew, ToolPackagesComposer};

fn generate_cargo(tool: &str) -> String {
    format!("( which cargo && ( cargo binstall {tool} || cargo install {tool} ) )")
}

fn generate_gem(tool: &str) -> String {
    format!("( which gem && gem install {tool} )")
}

fn generate_julia(tool: &str) -> String {
    format!("( which julia && julia  -e 'import Pkg; Pkg.add(\"{tool}\")' )")
}

fn generate_apt(tool: &str) -> String {
    format!("( which apt-get && sudo apt-get install -y {tool} )")
}

fn generate_go(tool: &str) -> String {
    format!("( which go && go install {tool} )")
}

fn generate_pip(tool: &str) -> String {
    format!("( which pipx && pipx install {tool} )")
}

fn generate_opam(tool: &str) -> String {
    format!("( which opam && eval $(opam env) && opam install {tool} )")
}

fn generate_dotnet(tool: &str) -> String {
    format!("( which dotnet && dotnet tool install -g {tool} )")
}

fn generate_brew(tool: &ToolPackagesBrew) -> String {
    let tap = tool
        .tap
        .as_ref()
        .map(|tap| format!("brew tap {tap} &&"))
        .unwrap_or_default();

    format!("( which brew && {} brew install {} )", tap, tool.name)
}

fn generate_luarocks(tool: &str) -> String {
    format!("( which luarocks && luarocks install {tool} )")
}

fn generate_cabal(tool: &str) -> String {
    format!("( which cabal && cabal install {tool} )")
}

fn generate_stack(tool: &str) -> String {
    format!("( which stack && stack install {tool} )")
}

fn generate_npm(tool: &str) -> String {
    format!("( which npm && npm i -g {tool} )")
}

fn generate_coursier(tool: &str) -> String {
    format!("( which cs && cs install {tool} )")
}

fn generate_nimble(tool: &str) -> String {
    format!("( which nimble && nimble install {tool} )")
}

fn generate_composer(tool: &ToolPackagesComposer) -> String {
    format!(
        "( which composer && composer require {} )",
        tool.package.as_ref().unwrap_or(&tool.binary)
    )
}

pub fn generate_install_steps(tools: &Vec<Tool>) -> Vec<WorkflowJobsStep> {
    let mut steps = Vec::new();

    for tool in tools {
        let mut has_tests = false;

        for options in tool.commands.values() {
            if !options.tests.is_empty() {
                has_tests = true;
                break;
            }
        }

        if !has_tests {
            continue;
        }

        if matches!(tool.disable_ci_tests, Some(true)) {
            continue;
        }

        let mut install_options = Vec::new();

        if let Some(npm) = &tool.packages.npm {
            install_options.push(generate_npm(npm));
        }

        if let Some(go) = &tool.packages.go {
            install_options.push(generate_go(go));
        }

        if let Some(cargo) = &tool.packages.cargo {
            install_options.push(generate_cargo(cargo));
        }

        if let Some(brew) = &tool.packages.brew {
            install_options.push(generate_brew(brew));
        }

        if let Some(apt) = &tool.packages.apt {
            install_options.push(generate_apt(apt));
        }

        if let Some(gem) = &tool.packages.gem {
            install_options.push(generate_gem(gem));
        }

        if let Some(pip) = &tool.packages.pip {
            install_options.push(generate_pip(pip));
        }

        if let Some(dotnet) = &tool.packages.dotnet {
            install_options.push(generate_dotnet(dotnet));
        }

        if let Some(luarocks) = &tool.packages.luarocks {
            install_options.push(generate_luarocks(luarocks));
        }

        if let Some(stack) = &tool.packages.stack {
            install_options.push(generate_stack(stack));
        }

        if let Some(cabal) = &tool.packages.cabal {
            install_options.push(generate_cabal(cabal));
        }

        if let Some(opam) = &tool.packages.opam {
            install_options.push(generate_opam(opam));
        }

        if let Some(coursier) = &tool.packages.coursier {
            install_options.push(generate_coursier(coursier));
        }

        if let Some(nimble) = &tool.packages.nimble {
            install_options.push(generate_nimble(nimble));
        }

        if let Some(composer) = &tool.packages.composer {
            install_options.push(generate_composer(composer));
        }

        if let Some(julia) = &tool.packages.julia {
            install_options.push(generate_julia(julia));
        }

        if install_options.is_empty() {
            println!("Missing install options for {}", tool.binary);
        } else {
            let binary_name = if ["dotnet", "julia"].contains(&tool.binary.as_str()) {
                tool.name.as_ref().unwrap_or(&tool.binary)
            } else {
                &tool.binary
            };

            let run = format!(
                "( ( which {binary_name} ) || ( {} ) || ( echo \"Unable to install tool\" ) )",
                install_options.join(" || ")
            );

            steps.push(WorkflowJobsStep {
                name: Some(tool.binary.clone()),
                run: Some(run),
                uses: None,
                with: None,
            });
        }
    }

    steps
}
