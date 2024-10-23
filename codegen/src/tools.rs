use convert_case::{Case, Casing};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct Tool {
    #[allow(unused)]
    name: Option<String>,

    binary: String,

    npm: Option<String>,

    php: Option<String>,

    #[expect(unused)]
    description: String,

    #[expect(unused)]
    website: String,

    #[expect(unused)]
    categories: std::collections::HashSet<String>,

    #[expect(unused)]
    languages: std::collections::HashSet<String>,

    commands: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct GeneratedCommand {
    enum_value: String,

    serde_value: String,

    fn_name: String,

    code: String,
}

impl Tool {
    fn generate(&self) -> Vec<GeneratedCommand> {
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

    for (index, cmd) in commands.iter().enumerate() {{
        let execution_result = execute_command({set_args_fn_name}(cmd.build(), file_path), file_path);

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
                fn_name: run_fn_name,
                code,
            });
        }

        all_commands
    }
}

pub fn generate() -> anyhow::Result<()> {
    let walker = ignore::WalkBuilder::new("tools").build().flatten();

    let folder = "mdsf/src/tools";

    let _ = std::fs::create_dir_all(folder);

    let mut files = std::collections::HashSet::new();

    let mut enum_values: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut format_snippet_values: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut as_ref_values: std::collections::HashSet<String> = std::collections::HashSet::new();

    for entry in walker {
        if entry.file_name() == "plugin.json" {
            let content = std::fs::read_to_string(entry.path())?;

            let parsed = serde_json::from_str::<Tool>(&content)?;

            println!("{parsed:#?}");

            let converted = parsed.generate();

            println!("{converted:#?}");

            for command in converted {
                std::fs::write(format!("{folder}/{}.rs", command.serde_value), command.code)?;

                files.insert(command.serde_value.clone());
                enum_values.insert(format!(
                    "    #[serde(rename = \"{}\")]
    {},",
                    command.serde_value, command.enum_value
                ));
                format_snippet_values.insert(format!(
                    "            Self::{} => {}::{}(snippet_path),",
                    command.enum_value, command.serde_value, command.fn_name
                ));

                as_ref_values.insert(format!(
                    "            Self::{} => \"{}\",",
                    command.enum_value, command.serde_value,
                ));
            }
        }
    }

    let mut modules = files
        .iter()
        .map(|module_name| format!("pub mod {module_name};"))
        .collect::<Vec<_>>();
    modules.sort_unstable();

    let mut tooling_enum_content = enum_values.into_iter().collect::<Vec<_>>();
    tooling_enum_content.sort_unstable();

    let mut format_snippet_content = format_snippet_values.into_iter().collect::<Vec<_>>();
    format_snippet_content.sort_unstable();

    let mut as_ref_content = as_ref_values.into_iter().collect::<Vec<_>>();
    as_ref_content.sort_unstable();

    let mod_file_contents = format!(
        "{}

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = \"json-schema\", derive(schemars::JsonSchema))]
pub enum Tooling {{
{}
}}

impl Tooling {{
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {{
        match self {{
{}
        }}
    }}
}}

impl AsRef<str> for Tooling {{
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {{
        match self {{
{}
        }}
    }}
}}
",
        modules.join("\n"),
        tooling_enum_content.join("\n"),
        format_snippet_content.join("\n"),
        as_ref_content.join("\n"),
    );

    std::fs::write(format!("{folder}/mod.rs"), mod_file_contents)?;

    Ok(())
}
