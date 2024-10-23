use convert_case::{Case, Casing};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct Tool {
    name: Option<String>,

    binary: String,

    npm: Option<String>,

    php: Option<String>,

    description: String,

    website: String,

    categories: std::collections::HashSet<String>,

    languages: std::collections::HashSet<String>,

    commands: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct GeneratedCommand {
    enum_value: String,

    serde_value: String,

    filename: String,

    code: String,
}

impl Tool {
    fn generate(&self) -> Vec<GeneratedCommand> {
        let mut commands: Vec<GeneratedCommand> = Vec::new();

        let binary_name = self.binary.clone();

        let fn_prefix = binary_name.to_case(Case::Snake);

        let mut all_commands = Vec::new();

        for (cmd, args) in &self.commands {
            let command_name = format!(
                "{fn_prefix}{}",
                if cmd.is_empty() {
                    String::new()
                } else {
                    format!("_{}", cmd.to_case(Case::Snake))
                }
            );

            let set_args_fn_name = format!("set_{command_name}_args");

            let run_fn_name = format!("run_{command_name}");

            let mut command_types: Vec<String> = Vec::new();
            {
                if let Some(npm) = &self.npm {
                    command_types.push(format!("CommandType::NodeModules(\"{}\")", npm));
                };

                if let Some(php) = &self.php {
                    command_types.push(format!("CommandType::PhpVendor(\"{}\")", php));
                };

                command_types.push(format!("CommandType::Direct(\"{}\")", self.binary));

                if let Some(npm) = &self.npm {
                    command_types.push(format!("CommandType::Npm(\"{npm}\")"));
                };
            };

            let command_arr = command_types.join(", ");

            let string_args = args
                .iter()
                .map(|arg| {
                    if arg == "$PATH" {
                        format!(".arg(file_path)")
                    } else {
                        format!(".arg(\"{arg}\")")
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            let code = format!(
                "use std::process::Command;

use crate::{{error::MdsfError, formatters::execute_command, runners::CommandType}};

#[inline]
fn {set_args_fn_name}(mut cmd: Command, file_path: &std::path::Path) -> Command {{
    cmd{string_args};

    cmd
}}

#[inline]
pub fn {run_fn_name}(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {{
    let commands = [{command_arr}];


    for (index, c) in commands.iter().enumerate() {{
        let mut cmd = c.build();

        let execution_result = execute_command({set_args_fn_name}(cmd, file_path), file_path);

        if index == commands.len() - 1 {{
            return execution_result;
        }}

        if let Ok(r) = execution_result {{
            if !r.0 {{
                return Ok(r);
            }}
        }}
    }}

    Ok((true, None))
}}
",
            );

            all_commands.push(GeneratedCommand {
                enum_value: command_name.clone().to_case(Case::Pascal),
                serde_value: command_name.clone().to_case(Case::Snake),
                code,
                filename: command_name.to_case(Case::Snake),
            });
        }

        all_commands
    }
}

pub fn generate() -> anyhow::Result<()> {
    let mut walker = ignore::WalkBuilder::new("tools").build();

    let folder = "mdsf/src/tools";

    let _ = std::fs::create_dir_all(folder);

    let mut modules = std::collections::HashSet::new();

    for entry in walker.flatten() {
        if entry.file_name() == "plugin.json" {
            let content = std::fs::read_to_string(entry.path())?;

            let parsed = serde_json::from_str::<Tool>(&content)?;

            println!("{parsed:#?}");

            let converted = parsed.generate();

            println!("{converted:#?}");

            for command in converted {
                std::fs::write(format!("{folder}/{}.rs", command.filename), command.code)?;

                modules.insert(command.filename);
            }
        }
    }

    let mut mod_file_contents = modules
        .iter()
        .map(|module_name| format!("pub mod {module_name};"))
        .collect::<Vec<_>>();

    mod_file_contents.sort_unstable();

    std::fs::write(format!("{folder}/mod.rs"), mod_file_contents.join("\n"))?;

    Ok(())
}
