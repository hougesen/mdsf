use std::hash::{DefaultHasher, Hash, Hasher};

use convert_case::{Case, Casing};

const INDENT: &str = "    ";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Hash)]
#[schemars(deny_unknown_fields)]
struct ToolTest {
    language: String,

    command: String,

    test_input: String,

    test_output: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Tool {
    #[expect(unused)]
    #[serde(rename = "$schema")]
    schema: String,

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

    tests: Option<Vec<ToolTest>>,
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
    fn get_command_name(&self, cmd: &str) -> String {
        let fn_prefix = self.name.as_ref().map_or_else(
            || self.binary.replace('.', "_").to_case(Case::Snake),
            |name| name.replace('.', "_").to_case(Case::Snake),
        );

        format!(
            "{fn_prefix}{}",
            if cmd.is_empty() {
                String::new()
            } else {
                format!("_{}", cmd.replace('.', "_").to_case(Case::Snake))
            }
        )
    }

    fn generate_test(&self, test: &ToolTest) -> (String, String) {
        let mut hasher = DefaultHasher::new();
        test.hash(&mut hasher);
        let id = format!("{:x}", hasher.finish());

        let module_name = self.get_command_name(&test.command).to_case(Case::Snake);

        let language = test.language.to_case(Case::Snake);

        let test_fn_name = format!("test_{module_name}_{language}_{id}",);

        let test_code = format!(
            "{INDENT}#[test_with::executable({bin})]
{INDENT}fn {test_fn_name}() {{
{INDENT}{INDENT}let input = r#\"{input}\"#;
{INDENT}{INDENT}let output = r#\"{output}\"#;
{INDENT}{INDENT}let file_ext = crate::fttype::get_file_extension(\"{language}\");
{INDENT}{INDENT}let snippet =
{INDENT}{INDENT}{INDENT}crate::execution::setup_snippet(input, &file_ext).expect(\"it to create a snippet file\");
{INDENT}{INDENT}let result = crate::tools::{module_name}::run(snippet.path())
{INDENT}{INDENT}{INDENT}.expect(\"it to be successful\")
{INDENT}{INDENT}{INDENT}.1
{INDENT}{INDENT}{INDENT}.expect(\"it to be some\");
{INDENT}{INDENT}assert_eq!(result, output);
{INDENT}}}",
            bin = if self.npm.is_some() {
                "npx"
            } else {
                &self.binary
            },
            input = test.test_input,
            output = test.test_output,
            language = test.language,
        );

        (module_name, test_code)
    }

    fn generate(&self) -> Vec<GeneratedCommand> {
        let mut all_commands = Vec::new();

        let mut generated_tests: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        if let Some(tests) = &self.tests {
            for test in tests {
                let (module, code) = self.generate_test(test);

                generated_tests.entry(module).or_default().push(code);
            }
        }

        for (cmd, args) in &self.commands {
            let command_name = self.get_command_name(cmd);

            let set_args_fn_name = format!("set_{command_name}_args");

            let run_fn_name = "run".to_string();

            let mut command_types: Vec<String> = Vec::new();
            {
                if self.npm.is_some() {
                    command_types.push(format!("CommandType::NodeModules(\"{}\")", self.binary));
                };

                if let Some(php) = &self.php {
                    command_types.push(format!("CommandType::PhpVendor(\"{php}\")"));
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

            let module_name = command_name.to_case(Case::Snake);

            let tests = generated_tests
                .remove(&module_name)
                .unwrap_or_default()
                .join("\n\n");

            let tests = if tests.is_empty() {
                String::new()
            } else {
                format!("\n{tests}\n")
            };

            let code = format!(
                "use std::process::Command;

use crate::{{error::MdsfError, execution::execute_command, runners::CommandType}};

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

#[cfg(test)]
mod test_{module_name} {{{tests}}}
",
            );

            all_commands.push(GeneratedCommand {
                enum_value: command_name.to_case(Case::Pascal),
                module_name,
                fn_name: run_fn_name,
                code,
                serde_rename: format!(
                    "{}{}{}",
                    self.name.as_ref().map_or(&self.binary, |n| n),
                    if cmd.is_empty() { "" } else { ":" },
                    cmd
                ),
            });
        }

        assert!(generated_tests.is_empty());

        all_commands
    }
}

pub fn generate() -> anyhow::Result<()> {
    let walker = ignore::WalkBuilder::new("tools").build().flatten();

    let folder = "mdsf/src/tools";

    std::fs::remove_dir_all(folder)?;

    let _ = std::fs::create_dir_all(folder);

    let clean_mod_file = "#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = \"json-schema\", derive(schemars::JsonSchema))]
pub enum Tooling {}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        &self,
        _snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        todo!()
    }
}

impl AsRef<str> for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {
        todo!()
    }
}
";

    std::fs::write(format!("{folder}/mod.rs"), clean_mod_file)?;

    let mut files = std::collections::HashSet::new();

    let mut enum_values: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut format_snippet_values: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut as_ref_values: std::collections::HashSet<String> = std::collections::HashSet::new();

    for entry in walker {
        if entry.file_name() == "plugin.json" {
            println!("{}", entry.path().display());

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
