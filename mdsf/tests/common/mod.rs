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

fn build_input_output(filetype: &str, input: &str, output: &str) -> (String, String) {
    if filetype == ".md" || get_file_extension(filetype) == ".md" {
        (input.to_string(), output.to_string())
    } else {
        (
            wrap_codeblock(filetype, input.trim()),
            wrap_codeblock(filetype, output.trim()),
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

fn run_verify_command(
    dir: &std::path::Path,
    file: &std::path::Path,
) -> Result<assert_cmd::Command, CargoError> {
    let mut cmd = assert_cmd::Command::cargo_bin("mdsf")?;

    cmd.current_dir(dir)
        .arg("verify")
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
    filetype: &str,
) -> Result<(), Box<dyn core::error::Error>> {
    let (input, expected_output) = build_input_output(filetype, input, expected_output);

    let dir = tempfile::TempDir::with_prefix("mdsf-tool-test")?;

    setup_config_file(dir.path(), filetype, tool)?;

    let file = setup_markdown_file(dir.path(), &input)?;

    // Validate input is correct
    assert_eq!(std::fs::read_to_string(file.path())?, input);

    {
        let cmd = run_verify_command(dir.path(), file.path())?.assert();

        dbg!(&cmd);

        if input == expected_output {
            cmd.success()
                .stderr(predicates::str::contains(" (unchanged)"));
        } else {
            cmd.failure()
                .stderr(predicates::str::contains(format!(
                    "{} has changes",
                    file.path().display(),
                )))
                .stderr(predicates::str::contains(
                    "Found changes while running in check mode (1 file)",
                ));
        }

        // Validate input was not changed
        assert_eq!(std::fs::read_to_string(file.path())?, input);
    };

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
