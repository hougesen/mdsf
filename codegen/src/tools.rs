use std::hash::{DefaultHasher, Hash, Hasher};

use convert_case::{Case, Casing};

const INDENT: &str = "    ";

const GENERATED_FILE_COMMENT: &str =
    "///\n/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY\n///";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Hash, Clone, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCommandTest {
    /// Codeblock language used when generating tests
    pub language: String,

    pub test_input: String,

    pub test_output: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCommand {
    pub arguments: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tests: Vec<ToolCommandTest>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesBrew {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tap: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesComposer {
    pub binary: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apt: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brew: Option<ToolPackagesBrew>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabal: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cargo: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coursier: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotnet: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub go: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub julia: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luarocks: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nimble: Option<String>,

    /// Name of package on npm, if published there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opam: Option<String>,

    /// Binary name if installed through composer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composer: Option<ToolPackagesComposer>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pip: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

fn default_tool_schema() -> String {
    "../tool.schema.json".to_owned()
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    #[serde(rename = "$schema", default = "default_tool_schema")]
    pub schema: String,

    pub binary: String,

    #[serde(default)]
    pub categories: std::collections::BTreeSet<String>,

    pub commands: std::collections::BTreeMap<String, ToolCommand>,

    #[serde(default)]
    pub description: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_ci_tests: Option<bool>,

    #[serde(default)]
    pub homepage: String,

    #[serde(default)]
    pub languages: std::collections::BTreeSet<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default)]
    pub packages: ToolPackages,
}

#[derive(Debug)]
pub struct GeneratedCommand {
    pub enum_value: String,

    pub module_name: String,

    pub code: String,

    pub serde_rename: String,

    pub binary: String,

    pub args: Vec<String>,
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

    fn generate_test(&self, command: &str, test: &ToolCommandTest) -> (String, String) {
        let mut hasher = DefaultHasher::new();

        test.hash(&mut hasher);

        let id = format!("{:x}", hasher.finish());

        let module_name = self.get_command_name(command).to_case(Case::Snake);

        let language = test.language.to_case(Case::Snake);

        let test_fn_name = format!("test_{module_name}_{language}_{id}",);

        let test_output = &test.test_output;

        let test_code = format!(
            "{INDENT}#[test_with::executable({bin})]
{INDENT}fn {test_fn_name}() {{
{INDENT}{INDENT}let input = r#\"{input}\"#;

{INDENT}{INDENT}let output = r#\"{test_output}\"#;

{INDENT}{INDENT}let file_ext = crate::fttype::get_file_extension(\"{language}\");

{INDENT}{INDENT}let snippet =
{INDENT}{INDENT}{INDENT}crate::execution::setup_snippet(input, &file_ext).expect(\"it to create a snippet file\");

{INDENT}{INDENT}let result =
{INDENT}{INDENT}{INDENT}crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_args, 0)
{INDENT}{INDENT}{INDENT}{INDENT}.expect(\"it to be successful\")
{INDENT}{INDENT}{INDENT}{INDENT}.1
{INDENT}{INDENT}{INDENT}{INDENT}.expect(\"it to be some\");

{INDENT}{INDENT}assert_eq!(result, output);
{INDENT}}}",
            bin = if self.packages.npm.is_some() {
                "npx"
            } else {
                &self.binary
            },
            input = test.test_input,
            language = test.language,
        );

        (module_name, test_code)
    }

    #[allow(clippy::too_many_lines)]
    fn generate(&self) -> Vec<GeneratedCommand> {
        let mut all_commands = Vec::new();

        for (cmd, options) in &self.commands {
            let command_name = self.get_command_name(cmd);

            let mut command_types: Vec<String> = Vec::new();
            {
                if self.packages.npm.is_some() {
                    command_types.push(format!("CommandType::NodeModules(\"{}\")", self.binary));
                };

                if let Some(php) = &self.packages.composer {
                    command_types.push(format!("CommandType::PhpVendor(\"{}\")", php.binary));
                };

                command_types.push(format!("CommandType::Direct(\"{}\")", self.binary));

                if let Some(npm) = &self.packages.npm {
                    command_types.push(format!("CommandType::Npm(\"{npm}\")"));
                };
            };

            // TODO: generate if statements instead of array
            let command_arr = if command_types.len() > 1 {
                format!(
                    "\n{INDENT}{},\n",
                    command_types.join(format!(",\n{INDENT}").as_str())
                )
            } else {
                command_types.join(", ")
            };
            let mut args_includes_path = false;

            let string_args = options
                .arguments
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

            let tests = options
                .tests
                .iter()
                .map(|test| self.generate_test(cmd, test).1)
                .collect::<Vec<_>>();

            let tests = if tests.is_empty() {
                String::new()
            } else {
                format!("\n{}\n", tests.join("\n\n"))
            };

            let command_type_count = command_types.len();

            let code = format!(
                "{GENERATED_FILE_COMMENT}
use crate::runners::CommandType;

#[inline]
pub fn set_args(
{INDENT}mut cmd: std::process::Command,
{INDENT}file_path: &std::path::Path,
) -> std::process::Command {{
{string_args}
{INDENT}cmd
}}

pub const COMMANDS: [CommandType; {command_type_count}] = [{command_arr}];

#[cfg(test)]
mod test_{module_name} {{{tests}}}
",
            );

            all_commands.push(GeneratedCommand {
                enum_value: command_name.to_case(Case::Pascal),
                module_name,
                code,
                serde_rename: format!(
                    "{}{}{}",
                    self.name.as_ref().map_or(&self.binary, |n| n),
                    if cmd.is_empty() { "" } else { ":" },
                    cmd
                ),
                args: options.arguments.clone(),
                binary: self.binary.clone(),
            });
        }

        all_commands
    }
}

#[allow(clippy::too_many_lines)]
pub fn generate(plugins: &[Tool]) -> anyhow::Result<Vec<GeneratedCommand>> {
    let folder = "mdsf/src/tools";

    std::fs::remove_dir_all(folder)?;

    let _ = std::fs::create_dir_all(folder);

    let clean_mod_file = "#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = \"json-schema\", derive(schemars::JsonSchema))]
pub enum Tooling {}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        self,
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

    let mut all_commands = Vec::new();

    for plugin in plugins {
        let converted = plugin.generate();

        for command in converted {
            std::fs::write(
                format!("{folder}/{}.rs", command.module_name),
                &command.code,
            )?;

            files.insert(command.module_name.clone());

            let enum_value = &command.enum_value;
            let module_name = &command.module_name;

            enum_values.insert(format!(
                "{INDENT}#[serde(rename = \"{rename}\")]
{INDENT}/// `{bin} {args}`
{INDENT}{enum_value},",
                rename = command.serde_rename,
                bin = plugin.binary,
                args = command.args.join(" ")
            ));

            format_snippet_values.insert(format!(
                "{INDENT}{INDENT}{INDENT}Self::{enum_value} => (&{module_name}::COMMANDS, {module_name}::set_args),"
             ));

            as_ref_values.insert(format!(
                "{INDENT}{INDENT}{INDENT}Self::{} => \"{}\",",
                command.enum_value, command.module_name,
            ));

            all_commands.push(command);
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
        "{GENERATED_FILE_COMMENT}
{}

#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = \"json-schema\", derive(schemars::JsonSchema))]
pub enum Tooling {{
{}
}}

impl Tooling {{
{INDENT}#[allow(clippy::too_many_lines)]
{INDENT}#[inline]
{INDENT}pub fn format_snippet(
{INDENT}{INDENT}self,
{INDENT}{INDENT}snippet_path: &std::path::Path,
{INDENT}{INDENT}timeout: u64,
{INDENT}) -> Result<(bool, Option<String>), crate::error::MdsfError> {{
{INDENT}{INDENT}let (commands, set_args_fn): (
{INDENT}{INDENT}{INDENT}&[crate::runners::CommandType],
{INDENT}{INDENT}{INDENT}fn(std::process::Command, &std::path::Path) -> std::process::Command,
{INDENT}{INDENT}) = match self {{
{}
{INDENT}{INDENT}}};

{INDENT}{INDENT}crate::execution::run_tools(commands, snippet_path, set_args_fn, timeout)
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

    Ok(all_commands)
}
