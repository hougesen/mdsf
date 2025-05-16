use std::io::Write;

use assert_cmd::cargo::CargoError;
use mdsf::{
    config::{MdsfConfig, MdsfConfigRunners},
    execution::MdsfFormatter,
    fttype::get_file_extension,
    tools::Tooling,
};

fn setup_config_file(
    dir: &std::path::Path,
    file_type: &str,
    tool: Tooling,
) -> Result<(), Box<dyn core::error::Error>> {
    let config = MdsfConfig {
        runners: MdsfConfigRunners::all(),
        languages: std::collections::BTreeMap::from_iter([(
            file_type.to_string(),
            MdsfFormatter::Single(tool),
        )]),
        format_finished_document: true,
        ..Default::default()
    };

    let contents = serde_json::to_string(&config)?;

    let p = dir.join("mdsf.json");

    std::fs::write(&p, &contents)?;

    Ok(())
}

fn wrap_codeblock(ft: &str, code: &str) -> String {
    format!(
        "```{ft}
{code}
```
"
    )
}

fn build_input_output(file_type: &str, input: &str, output: &str) -> (String, String) {
    if file_type == ".md" || get_file_extension(file_type) == ".md" {
        (input.to_string(), output.to_string())
    } else {
        (
            wrap_codeblock(file_type, input.trim()),
            wrap_codeblock(file_type, output.trim()),
        )
    }
}

fn setup_markdown_file(
    dir: &std::path::Path,
    input: &str,
) -> std::io::Result<tempfile::NamedTempFile> {
    let mut b = tempfile::Builder::new();

    b.prefix("mdsf-test").rand_bytes(18).suffix(".md");

    let mut f = b.tempfile_in(dir)?;

    f.write_all(input.as_bytes())?;
    f.flush()?;

    Ok(f)
}

fn run_format_command(
    dir: &std::path::Path,
    file: &std::path::Path,
) -> Result<assert_cmd::Command, CargoError> {
    let mut cmd = assert_cmd::Command::cargo_bin("mdsf")?;

    cmd.current_dir(dir)
        .arg("format")
        .arg("--debug")
        .arg("--log-level")
        .arg("trace")
        .arg(file);

    Ok(cmd)
}

pub fn run_tooling_test(
    tool: Tooling,
    input: &str,
    expected_output: &str,
    file_ext: &str,
) -> Result<(), Box<dyn core::error::Error>> {
    let (input, expected_output) = build_input_output(file_ext, input, expected_output);

    let dir = tempfile::TempDir::with_prefix("mdsf-tool-test")?;

    setup_config_file(dir.path(), file_ext, tool)?;

    let file = setup_markdown_file(dir.path(), &input)?;

    // Validate input is correct
    assert_eq!(std::fs::read_to_string(file.path())?, input);

    {
        run_format_command(dir.path(), file.path())?
            .assert()
            .success();

        let output = std::fs::read_to_string(file.path())?;

        assert_eq!(output.trim(), expected_output.trim());
    };

    drop(dir);

    Ok(())
}
