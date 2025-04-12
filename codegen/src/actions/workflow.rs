#[derive(serde::Serialize)]
pub struct WorkflowConcurrency {
    pub group: String,
    #[serde(rename = "cancel-in-progress")]
    pub cancel_in_progress: bool,
}

#[derive(serde::Serialize)]
pub struct WorkflowEnv {
    #[serde(rename = "CARGO_TERM_COLOR")]
    pub cargo_term_color: String,
    #[serde(rename = "RUST_BACKTRACE")]
    pub rust_backtrace: String,
}

#[derive(serde::Serialize)]
pub struct WorkflowJobsStrategyMatrix {
    pub os: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct WorkflowJobsStrategy {
    pub matrix: WorkflowJobsStrategyMatrix,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WorkflowJobsStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<std::collections::BTreeMap<String, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,

    #[serde(rename = "continue-on-error", skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct WorkflowJobs {
    pub name: String,
    pub strategy: WorkflowJobsStrategy,
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<WorkflowJobsStep>,
}

#[derive(serde::Serialize)]
pub struct Workflow {
    pub name: String,
    pub on: Vec<String>,
    pub concurrency: WorkflowConcurrency,
    pub env: WorkflowEnv,
    pub jobs: std::collections::BTreeMap<String, WorkflowJobs>,
}
