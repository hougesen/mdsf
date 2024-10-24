use convert_case::{Case, Casing};

const INDENT: &str = "    ";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct ToolTest {
    #[expect(unused)]
    language: String,

    #[expect(unused)]
    command: String,

    #[expect(unused)]
    test_input: String,

    #[expect(unused)]
    test_output: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Tool {
    name: Option<String>,

    binary: String,

    npm: Option<String>,

    php: Option<String>,

    commands: std::collections::HashMap<String, Vec<String>>,

    description: String,

    homepage: String,

    #[expect(unused)]
    categories: std::collections::HashSet<String>,

    #[expect(unused)]
    languages: std::collections::HashSet<String>,

    #[expect(unused)]
    tests: Vec<ToolTest>,
}

#[derive(Debug)]
struct GeneratedCommand {
    enum_value: String,

    module_name: String,

    fn_name: String,

    code: String,

    serde_rename: String,
}

impl Tool {
    fn generate(&self) -> Vec<GeneratedCommand> {
        let fn_prefix = if let Some(name) = &self.name {
            name.replace(".", "_").to_case(Case::Snake)
        } else {
            self.binary.replace(".", "_").to_case(Case::Snake)
        };

        let mut all_commands = Vec::new();

        for (cmd, args) in &self.commands {
            let command_name = format!(
                "{fn_prefix}{}",
                if cmd.is_empty() {
                    String::new()
                } else {
                    format!("_{}", cmd.replace(".", "_").to_case(Case::Snake))
                }
            );

            let set_args_fn_name = format!("set_{command_name}_args");

            let run_fn_name = "run".to_string();

            let mut command_types: Vec<String> = Vec::new();
            {
                if self.npm.is_some() {
                    command_types.push(format!("CommandType::NodeModules(\"{}\")", self.binary));
                };

                if let Some(php) = &self.php {
                    command_types.push(format!("CommandType::PhpVendor(\"{}\")", php));
                };

                command_types.push(format!("CommandType::Direct(\"{}\")", self.binary));

                if let Some(npm) = &self.npm {
                    command_types.push(format!("CommandType::Npm(\"{npm}\")"));
                };
            };

            // TODO: generate if statements instead of array
            let command_arr = if command_types.len() > 1 {
                format!(
                    "\n{INDENT}{INDENT}{},\n{INDENT}",
                    command_types.join(format!(",\n{INDENT}{INDENT}").as_str())
                )
            } else {
                command_types.join(", ")
            };
            let mut args_includes_path = false;

            let string_args = args
                .iter()
                .map(|arg| {
                    if arg == "$PATH" {
                        args_includes_path = true;
                        format!("{INDENT}cmd.arg(file_path);")
                    } else if arg == "$PATH_STRING" {
                        args_includes_path = true;

                        let arg_str = "cmd.arg(format!(\"'{}'\", file_path.to_string_lossy()));";
                        format!("{INDENT}{arg_str}")
                    } else if arg.contains("$PATH_STRING") {
                        args_includes_path = true;

                        let declaration = "let fps = file_path.to_string_lossy();\n";

                        format!(
                            "{INDENT}{declaration}{INDENT}cmd.arg(format!(\"{}\"));",
                            arg.replace("$PATH_STRING", "fps")
                        )
                    } else {
                        format!("{INDENT}cmd.arg(\"{arg}\");",)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            assert!(args_includes_path);

            let code = format!(
                "use std::process::Command;

use crate::{{error::MdsfError, formatters::execute_command, runners::CommandType}};

#[inline]
fn {set_args_fn_name}(mut cmd: Command, file_path: &std::path::Path) -> Command {{
{string_args}
{INDENT}cmd
}}

#[inline]
pub fn {run_fn_name}(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {{
{INDENT}let commands = [{command_arr}];

{INDENT}for (index, cmd) in commands.iter().enumerate() {{
{INDENT}{INDENT}let cmd = {set_args_fn_name}(cmd.build(), file_path);
{INDENT}{INDENT}let execution_result = execute_command(cmd, file_path);

{INDENT}{INDENT}if index == commands.len() - 1 {{
{INDENT}{INDENT}{INDENT}return execution_result;
{INDENT}{INDENT}}}

{INDENT}{INDENT}if let Ok(r) = execution_result {{
{INDENT}{INDENT}{INDENT}if !r.0 {{
{INDENT}{INDENT}{INDENT}{INDENT}return Ok(r);
{INDENT}{INDENT}{INDENT}}}
{INDENT}{INDENT}}}
{INDENT}}}

{INDENT}Ok((true, None))
}}
",
            );

            all_commands.push(GeneratedCommand {
                enum_value: command_name.to_case(Case::Pascal),
                module_name: command_name.to_case(Case::Snake),
                fn_name: run_fn_name,
                code,
                serde_rename: format!(
                    "{}{}{}",
                    if let Some(n) = &self.name {
                        n
                    } else {
                        &self.binary
                    },
                    if cmd.is_empty() { "" } else { ":" },
                    cmd
                ),
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
            println!("{entry:?}");
            let content = std::fs::read_to_string(entry.path())?;

            let parsed = serde_json::from_str::<Tool>(&content)?;

            let converted = parsed.generate();

            for command in converted {
                std::fs::write(format!("{folder}/{}.rs", command.module_name), command.code)?;

                files.insert(command.module_name.clone());
                enum_values.insert(format!(
                    "{INDENT}#[serde(rename = \"{rename}\")]
{INDENT}#[doc = \"{description} - [{homepage}]({homepage})\"]
{INDENT}{enum_name},",
                    rename = command.serde_rename,
                    enum_name = command.enum_value,
                    description = parsed.description,
                    homepage = parsed.homepage,
                ));
                format_snippet_values.insert(format!(
                    "{INDENT}{INDENT}{INDENT}Self::{} => {}::{}(snippet_path),",
                    command.enum_value, command.module_name, command.fn_name
                ));

                as_ref_values.insert(format!(
                    "{INDENT}{INDENT}{INDENT}Self::{} => \"{}\",",
                    command.enum_value, command.module_name,
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
{INDENT}#[allow(clippy::too_many_lines)]
{INDENT}#[inline]
{INDENT}pub fn format_snippet(
{INDENT}{INDENT}&self,
{INDENT}{INDENT}snippet_path: &std::path::Path,
{INDENT}) -> Result<(bool, Option<String>), crate::error::MdsfError> {{
{INDENT}{INDENT}match self {{
{}
{INDENT}{INDENT}}}
{INDENT}}}
}}

impl AsRef<str> for Tooling {{
{INDENT}#[allow(clippy::too_many_lines)]
{INDENT}#[inline]
{INDENT}fn as_ref(&self) -> &str {{
{INDENT}{INDENT}match self {{
{}
{INDENT}{INDENT}}}
{INDENT}}}
}}
",
        modules.join("\n"),
        tooling_enum_content.join("\n\n"),
        format_snippet_content.join("\n"),
        as_ref_content.join("\n"),
    );

    std::fs::write(format!("{folder}/mod.rs"), mod_file_contents)?;

    Ok(())
}
