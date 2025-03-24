use workflow::{
    Workflow, WorkflowConcurrency, WorkflowEnv, WorkflowJobs, WorkflowJobsStep,
    WorkflowJobsStrategy, WorkflowJobsStrategyMatrix,
};

use crate::tools::Tool;

mod packages;
mod workflow;

#[derive(serde::Deserialize)]
struct BaseSteps {
    uses: Vec<WorkflowJobsStep>,

    #[serde(rename = "post:uses")]
    post_uses: Vec<WorkflowJobsStep>,

    #[serde(rename = "post:packages")]
    post_packages: Vec<WorkflowJobsStep>,
}

fn get_base_steps() -> anyhow::Result<BaseSteps> {
    let contents = serde_json::from_str::<BaseSteps>(&std::fs::read_to_string(
        "./codegen/test-workflow.json",
    )?)?;

    Ok(contents)
}

pub fn generate(plugins: &Vec<Tool>) -> anyhow::Result<()> {
    let mut steps: Vec<workflow::WorkflowJobsStep> = Vec::new();

    let mut base_steps = get_base_steps()?;

    steps.append(&mut base_steps.uses);

    steps.append(&mut base_steps.post_uses);

    steps.append(&mut packages::generate_install_steps(plugins));

    steps.append(&mut base_steps.post_packages);

    let workflow = Workflow {
        name: "test".to_owned(),
        on: vec!["push".to_owned(), "workflow_dispatch".to_owned()],
        concurrency: WorkflowConcurrency {
            group: r"${{ github.workflow }}-${{ github.head_ref || github.run_id }}".to_owned(),
            cancel_in_progress: true,
        },
        env: WorkflowEnv {
            cargo_term_color: "always".to_owned(),
            rust_backtrace: "full".to_owned(),
        },
        jobs: std::collections::BTreeMap::from_iter([(
            "tools".to_owned(),
            WorkflowJobs {
                name: "tools".to_owned(),
                strategy: WorkflowJobsStrategy {
                    matrix: WorkflowJobsStrategyMatrix {
                        os: vec!["ubuntu-latest".to_string()],
                    },
                },
                runs_on: "${{ matrix.os }}".to_owned(),
                steps,
            },
        )]),
    };

    std::fs::write(
        ".github/workflows/test.yml",
        serde_yaml::to_string(&workflow)?,
    )?;

    Ok(())
}
