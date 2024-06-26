use crate::readme_tooling::update_readme;

fn execute_command(name: &str) -> std::io::Result<std::process::Output> {
    let mut x = std::process::Command::new("cargo");
    x.arg("build");
    x.current_dir("../");
    x.output()?;

    std::process::Command::new("../target/debug/mdsf")
        .arg(name)
        .arg("--help")
        .output()
}

fn update_command(name: &str) -> anyhow::Result<()> {
    println!("generate help for command '{name}'");

    let help = String::from_utf8(execute_command(name)?.stdout)?;

    update_readme(
        &format!("{name}-command-help"),
        &format!("```\n{}\n```", help.trim()),
    )?;

    anyhow::Ok(())
}

pub fn generate() -> anyhow::Result<()> {
    update_command("format")?;

    update_command("verify")?;

    update_command("completions")?;

    anyhow::Ok(())
}
