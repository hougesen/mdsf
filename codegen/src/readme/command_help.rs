use crate::markdown::update_markdown_section;

fn execute_command(name: &str) -> std::io::Result<std::process::Output> {
    std::process::Command::new("cargo").arg("build").output()?;

    let mut help_command = std::process::Command::new("./target/debug/mdsf");

    if !name.is_empty() {
        help_command.arg(name);
    }

    help_command.arg("--help");

    help_command.output()
}

pub fn generate(mut readme: String) -> anyhow::Result<String> {
    for command in ["", "format", "verify", "completions"] {
        let section_name = &format!(
            "{}-command-help",
            if command.is_empty() { "base" } else { command }
        );

        println!("generate help for command '{section_name}'");

        let help = String::from_utf8(execute_command(command)?.stdout)?;

        readme =
            update_markdown_section(&readme, section_name, &format!("```\n{}\n```", help.trim()));
    }

    anyhow::Ok(readme)
}
