use super::workflow::WorkflowJobsStep;
use crate::tools::{Tool, ToolPackagesBrew};

fn generate_cargo(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which cargo && cargo install {tool} )"))
}

fn generate_gem(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which gem && gem install {tool} )"))
}

fn generate_julia(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which julia && julia  -e 'import Pkg; Pkg.add(\"{tool}\")' )"))
}

fn generate_apt(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which apt-get && sudo apt-get install -y {tool} )"))
}

fn generate_go(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which go && go install {tool} )"))
}

fn generate_pip(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which pipx && pipx install {tool} )"))
}

fn generate_opam(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which opam && eval $(opam env) && opam install {tool} )"))
}

fn generate_dotnet(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which dotnet && dotnet tool install -g {tool} )"))
}

fn generate_brew(tool: &Option<ToolPackagesBrew>) -> Option<String> {
    tool.as_ref().map(|tool| {
        let tap = tool
            .tap
            .as_ref()
            .map(|tap| format!("brew tap {tap} && "))
            .unwrap_or_default();

        format!("( which brew && {} brew install {} )", tap, tool.name)
    })
}

fn generate_luarocks(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which luarocks && luarocks install {tool} )"))
}

fn generate_cabal(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which cabal && cabal install {tool} )"))
}

fn generate_stack(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which stack && stack install {tool} )"))
}

fn generate_npm(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!("( which npm && npm i -g {tool} )"))
}

fn generate_coursier(tool: &Option<String>) -> Option<String> {
    tool.as_ref()
        .map(|tool| format!(" (which cs && cs install {tool} )"))
}

pub fn generate_install_steps(tools: &Vec<Tool>) -> anyhow::Result<Vec<WorkflowJobsStep>> {
    let mut steps = Vec::new();

    for tool in tools {
        let mut has_tests = false;

        for options in tool.commands.values() {
            if options
                .tests
                .as_ref()
                .is_some_and(|tests| !tests.is_empty())
            {
                has_tests = true;
                break;
            }
        }

        if !has_tests {
            continue;
        }

        if matches!(tool.testing_disable, Some(true)) {
            continue;
        }

        let run = generate_npm(&tool.packages.npm)
            .or_else(|| generate_go(&tool.packages.go))
            .or_else(|| generate_cargo(&tool.packages.cargo))
            .or_else(|| generate_brew(&tool.packages.brew))
            .or_else(|| generate_apt(&tool.packages.apt))
            .or_else(|| generate_gem(&tool.packages.gem))
            .or_else(|| generate_pip(&tool.packages.pip))
            .or_else(|| generate_dotnet(&tool.packages.dotnet))
            .or_else(|| generate_luarocks(&tool.packages.luarocks))
            .or_else(|| generate_stack(&tool.packages.stack))
            .or_else(|| generate_cabal(&tool.packages.cabal))
            .or_else(|| generate_opam(&tool.packages.opam))
            .or_else(|| generate_coursier(&tool.packages.coursier))
            .or_else(|| generate_julia(&tool.packages.julia));

        if run.is_none() {
            println!("Missing package for {}", tool.binary);
        }

        if let Some(run) = run {
            if run.starts_with("npm") {
                // We are skipping npm for now
                continue;
            }

            steps.push(WorkflowJobsStep {
                name: Some(tool.binary.clone()),
                run: Some(run),
                uses: None,
                with: None,
            });
        }
    }

    Ok(steps)
}
