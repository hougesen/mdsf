use crate::readme_tooling::update_readme;

fn execute_command(name: &str) -> std::io::Result<std::process::Output> {
    std::process::Command::new("cargo").arg("build").output()?;

    let mut help_command = std::process::Command::new("./target/debug/mdsf");

    if !name.is_empty() {
        help_command.arg(name);
    }

    help_command.arg("--help");

    help_command.output()
}

fn update_command(name: &str) -> anyhow::Result<()> {
    let section_name = &format!(
        "{}-command-help",
        if name.is_empty() { "base" } else { name }
    );

    println!("generate help for command '{section_name}'");

    let help = String::from_utf8(execute_command(name)?.stdout)?;

    update_readme(section_name, &format!("```\n{}\n```", help.trim()))?;

    anyhow::Ok(())
}

pub fn generate() -> anyhow::Result<()> {
    update_command("")?;

    update_command("format")?;

    update_command("verify")?;

    update_command("completions")?;

    anyhow::Ok(())
}
