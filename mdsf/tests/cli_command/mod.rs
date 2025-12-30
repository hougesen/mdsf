use std::io::Write as _;

pub fn cache_prune_command() -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("mdsf");
    cmd.arg("cache-prune");
    cmd
}

pub fn completions_command() -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("mdsf");
    cmd.arg("completions");
    cmd
}

pub fn init_command(dir: &std::path::Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("mdsf");
    cmd.arg("init").current_dir(dir);
    cmd
}

pub fn verify_command(dir: &std::path::Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("mdsf");
    cmd.arg("verify").current_dir(dir);
    cmd
}

pub fn format_command(dir: &std::path::Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo_bin_cmd!("mdsf");
    cmd.arg("format").current_dir(dir);
    cmd
}

pub fn setup_test_dir() -> Result<tempfile::TempDir, std::io::Error> {
    tempfile::TempDir::with_prefix("mdsf")
}

pub fn setup_test_input(
    dir: &std::path::Path,
    code: &str,
) -> Result<tempfile::NamedTempFile, std::io::Error> {
    let mut b = tempfile::Builder::new();

    b.rand_bytes(12).suffix(".md");

    let mut f = b.tempfile_in(dir)?;

    f.write_all(code.as_bytes())?;
    f.flush()?;

    Ok(f)
}

pub const BROKEN_CODE: &str = "```rust
fn add(a:i32,b:i32)->i32{a+b}
```
";

pub const FORMATTED_CODE: &str = "```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
";
