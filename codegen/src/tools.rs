use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

use convert_case::{Case, Casing};

use crate::GENERATED_FILE_COMMENT;

const INDENT: &str = "    ";

#[allow(clippy::trivially_copy_pass_by_ref)]
#[inline]
const fn is_false(b: &bool) -> bool {
    !(*b)
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Hash, Clone, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCommandTest {
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,

    /// Codeblock language used when generating tests
    pub language: String,

    pub test_input: String,

    pub test_output: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCommand {
    pub arguments: Vec<String>,

    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(url)]
    pub homepage: Option<String>,

    #[serde(default, skip_serializing_if = "is_false")]
    pub stdin: bool,

    #[serde(default)]
    pub tests: Vec<ToolCommandTest>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesApt {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesBrew {
    #[serde(default, skip_serializing_if = "is_false")]
    pub cask: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub skip_brew_install: bool,

    pub package: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tap: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesCabal {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesCargo {
    pub package: String,
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
pub struct ToolPackagesCoursier {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesDotnet {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesDub {
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_dub_run: bool,

    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesGem {
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_gem_exec: bool,

    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesGo {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesJulia {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesLuarocks {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesNimble {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct ToolPackagesNpm {
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_bunx: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_deno_run: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_npx: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_pnpm_dlx: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_yarn_exec: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,

    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesOpam {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesPip {
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_pipx_run: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_uv_tool_run: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,

    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackagesStack {
    pub package: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, Clone, Default, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPackages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apt: Option<ToolPackagesApt>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brew: Option<ToolPackagesBrew>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabal: Option<ToolPackagesCabal>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cargo: Option<ToolPackagesCargo>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coursier: Option<ToolPackagesCoursier>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotnet: Option<ToolPackagesDotnet>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dub: Option<ToolPackagesDub>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem: Option<ToolPackagesGem>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub go: Option<ToolPackagesGo>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub julia: Option<ToolPackagesJulia>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luarocks: Option<ToolPackagesLuarocks>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nimble: Option<ToolPackagesNimble>,

    /// Name of package on npm, if published there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm: Option<ToolPackagesNpm>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opam: Option<ToolPackagesOpam>,

    /// Binary name if installed through composer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composer: Option<ToolPackagesComposer>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pip: Option<ToolPackagesPip>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<ToolPackagesStack>,
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

    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,

    #[serde(default)]
    pub description: String,

    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_ci_package_install: bool,

    #[serde(default)]
    #[schemars(url)]
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

    pub description: String,

    pub homepage: String,

    pub deprecated: bool,

    pub tests: String,
}

const DEPRECATED_ATTRIBUTE: &str = "\n    #[deprecated]";

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
                format!("_{}", cmd.replace(['.', ':'], "_").to_case(Case::Snake))
            }
        )
    }

    fn generate_test(&self, command: &str, test: &ToolCommandTest) -> String {
        let mut hasher = DefaultHasher::new();

        test.language.hash(&mut hasher);
        test.test_input.hash(&mut hasher);
        test.test_output.hash(&mut hasher);

        let id = format!("{:x}", hasher.finish());

        let module_name = self.get_command_name(command).to_case(Case::Snake);
        let enum_value = self.get_command_name(command).to_case(Case::Pascal);

        let test_fn_name = format!(
            "test_{module_name}_{}_{id}",
            test.language.to_case(Case::Snake).replace('.', "")
        );

        let mut test_with_binaries = vec![self.binary.as_str()];

        if let Some(npm) = self.packages.npm.as_ref() {
            if !npm.disable_bunx {
                test_with_binaries.push("bunx");
            }

            if !npm.disable_deno_run {
                test_with_binaries.push("deno");
            }

            if !npm.disable_npx {
                test_with_binaries.push("npx");
            }

            if !npm.disable_pnpm_dlx {
                test_with_binaries.push("pnpm");
            }

            if !npm.disable_yarn_exec {
                test_with_binaries.push("yarn");
            }
        }

        if let Some(pip) = self.packages.pip.as_ref() {
            if !pip.disable_pipx_run {
                test_with_binaries.push("pipx");
            }

            if !pip.disable_uv_tool_run {
                test_with_binaries.push("uv");
            }
        }

        if self
            .packages
            .dub
            .as_ref()
            .is_some_and(|package| !package.disable_dub_run)
        {
            test_with_binaries.push("dub");
        }

        if self
            .packages
            .gem
            .as_ref()
            .is_some_and(|gem| !gem.disable_gem_exec)
        {
            test_with_binaries.push("gem");
        }

        let executable = test_with_binaries.join(" || ");

        let disable_attribute = if test.disabled {
            format!("{INDENT}#[ignore = \"Disabled in plugin file\"]\n")
        } else {
            String::new()
        };

        let test_input = &test.test_input;

        let test_output = &test.test_output;

        let test_language = &test.language;

        let test_code = format!(
            "{disable_attribute}{INDENT}#[test_with::executable({executable})]
{INDENT}fn {test_fn_name}() -> Result<(), Box<dyn core::error::Error>> {{
{INDENT}{INDENT}let input = r#\"{test_input}\"#;

{INDENT}{INDENT}let output = r#\"{test_output}\"#;

{INDENT}{INDENT}let ft = \"{test_language}\";

{INDENT}{INDENT}crate::common::run_tooling_test(crate::config::MdsfTool::Preset(mdsf::tools::Tooling::{enum_value}), input, output, ft)
{INDENT}}}",
        );

        test_code
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn generate(&self) -> Vec<GeneratedCommand> {
        let mut all_commands = Vec::new();

        for (cmd, options) in &self.commands {
            let command_name = self.get_command_name(cmd);

            let mut command_types: Vec<String> = Vec::new();
            {
                if self.packages.npm.is_some() {
                    command_types.push(format!("CommandType::NodeModules(\"{}\")", self.binary));
                }

                if let Some(php) = &self.packages.composer {
                    command_types.push(format!("CommandType::PhpVendor(\"{}\")", php.binary));
                }

                if let Some(dotnet) = &self.packages.dotnet {
                    command_types.push(format!("CommandType::Dotnet(\"{}\")", &dotnet.package));
                }

                command_types.push(format!("CommandType::Direct(\"{}\")", self.binary));

                if let Some(npm) = &self.packages.npm {
                    let package_name = &npm.package;
                    let executable_name = npm.executable.as_ref().unwrap_or(&npm.package);

                    let mut flavors = Vec::new();

                    if !npm.disable_npx {
                        flavors.push("Npm");
                    }

                    if !npm.disable_pnpm_dlx {
                        flavors.push("Pnpm");
                    }

                    if !npm.disable_bunx {
                        flavors.push("Bun");
                    }

                    if !npm.disable_deno_run {
                        flavors.push("Deno");
                    }

                    if !npm.disable_yarn_exec {
                        flavors.push("Yarn");
                    }

                    for flavor in flavors {
                        command_types.push(format!(
                            "CommandType::{flavor}(\"{package_name}\", \"{executable_name}\")"
                        ));
                    }
                }

                if let Some(pip) = &self.packages.pip {
                    let package_name = &pip.package;
                    let executable_name = pip.executable.as_ref().unwrap_or(&pip.package);

                    let mut flavors = Vec::new();

                    if !pip.disable_uv_tool_run {
                        flavors.push("Uv");
                    }

                    if !pip.disable_pipx_run {
                        flavors.push("Pipx");
                    }

                    for flavor in flavors {
                        command_types.push(format!(
                            "CommandType::{flavor}(\"{package_name}\", \"{executable_name}\")"
                        ));
                    }
                }

                if let Some(dub) = &self.packages.dub
                    && !dub.disable_dub_run
                {
                    command_types.push(format!("CommandType::Dub(\"{}\")", &dub.package));
                }

                if let Some(gem) = &self.packages.gem
                    && !gem.disable_gem_exec
                {
                    command_types.push(format!("CommandType::GemExec(\"{}\")", &gem.package));
                }
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
                        format!("{INDENT}cmd.arg(\"{arg}\");")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            if !options.stdin {
                assert!(args_includes_path);
            }

            let module_name = command_name.to_case(Case::Snake);

            let command_type_count = command_types.len();

            let serde_rename = format!(
                "{}{}{}",
                self.name.as_ref().map_or(&self.binary, |n| n),
                if cmd.is_empty() { "" } else { ":" },
                cmd
            );

            if options.tests.is_empty() {
                println!("Missing tests for: '{serde_rename}'");
            }

            let test_mod = format!(
                "#[cfg(test)]
mod test_{module_name} {{{maybe_line_break}{tests}{maybe_line_break}}}
",
                maybe_line_break = if options.tests.is_empty() { "" } else { "\n" },
                tests = options
                    .tests
                    .iter()
                    .map(|test| self.generate_test(cmd, test))
                    .collect::<Vec<_>>()
                    .join("\n\n")
            );

            let code = format!(
                "{GENERATED_FILE_COMMENT}

use crate::runners::CommandType;

#[inline]
pub {set_args_is_const}fn set_args(
{INDENT}{is_mut}cmd: std::process::Command,
{INDENT}{unused_prefix}file_path: &std::path::Path,
) -> std::process::Command {{
{string_args}
{INDENT}cmd
}}

pub const COMMANDS: [CommandType; {command_type_count}] = [{command_arr}];

pub const IS_STDIN: bool = {is_stdin};
",
                set_args_is_const = if options.arguments.is_empty() {
                    "const "
                } else {
                    ""
                },
                unused_prefix = if args_includes_path { "" } else { "_" },
                is_mut = if string_args.is_empty() { "" } else { "mut " },
                is_stdin = if options.stdin { "true" } else { "false" }
            );

            let homepage = options.homepage.clone().unwrap_or_default();

            let description = options.description.clone().unwrap_or_default();

            all_commands.push(GeneratedCommand {
                enum_value: command_name.to_case(Case::Pascal),
                module_name,
                code,
                serde_rename,
                args: options.arguments.clone(),
                binary: self.binary.clone(),
                homepage: if homepage.is_empty() {
                    self.homepage.clone()
                } else {
                    homepage
                },
                description: if description.is_empty() {
                    self.description.clone()
                } else {
                    description
                },
                deprecated: options.deprecated || self.deprecated,
                tests: test_mod,
            });
        }

        all_commands
    }
}

fn generate_reversible_tooling_test(values: std::collections::BTreeSet<String>) -> String {
    let mut asserts = values
        .into_iter()
        .map(|value| {
            format!("{INDENT}{INDENT}assert_eq!(Tooling::{value}, reverse(Tooling::{value})?);")
        })
        .collect::<Vec<_>>();
    asserts.sort_unstable();

    let asserts = asserts.join("\n");

    format!(
        "#[cfg(test)]
mod test_tooling {{
{INDENT}use super::Tooling;

{INDENT}fn reverse(tooling: Tooling) -> Result<Tooling, serde_json::Error> {{
{INDENT}{INDENT}serde_json::from_str::<Tooling>(&format!(\"\\\"{{}}\\\"\", tooling.as_ref()))
{INDENT}}}

{INDENT}#[allow(clippy::cognitive_complexity)]
{INDENT}#[allow(clippy::too_many_lines)]
{INDENT}#[test]
{INDENT}fn value_is_reversible() -> Result<(), serde_json::Error> {{
{asserts}

{INDENT}{INDENT}Ok(())
{INDENT}}}
}}
"
    )
}

#[allow(clippy::too_many_lines)]
pub fn generate(plugins: &[Tool]) -> Result<Vec<GeneratedCommand>, std::io::Error> {
    let folder = "mdsf/src/tools";

    std::fs::remove_dir_all(folder)?;

    let _ = std::fs::create_dir_all(folder);

    let clean_mod_file =
        "#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = \"json-schema\", derive(schemars::JsonSchema))]
pub enum Tooling {}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        self,
        snippet_path: &std::path::Path,
        timeout: u64,
        debug_enabled: bool,
        config_runners: &crate::config::MdsfConfigRunners,
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

    let mut enum_value_names = std::collections::BTreeSet::new();

    let mut all_tests = Vec::new();

    for plugin in plugins {
        let converted = plugin.generate();

        for command in converted {
            all_tests.push(command.tests.clone());

            std::fs::write(
                format!("{folder}/{}.rs", command.module_name),
                &command.code,
            )?;

            files.insert(command.module_name.clone());

            let enum_value = &command.enum_value;
            let module_name = &command.module_name;

            enum_value_names.insert(enum_value.clone());

            let homepage = if command.homepage.is_empty() {
                String::new()
            } else {
                format!(
                    "{INDENT}/// [{}]({})
{INDENT}///",
                    command.homepage, command.homepage
                )
            };

            let description = if command.description.is_empty() {
                String::new()
            } else {
                format!(
                    "{INDENT}/// {}
{INDENT}///",
                    command.description.trim()
                )
            };

            enum_values.insert(format!(
                "{INDENT}#[serde(rename = \"{rename}\")]{maybe_deprecated}
{description}
{homepage}
{INDENT}/// `{bin}{space_if_args}{args}`
{INDENT}{enum_value},",
                rename = command.serde_rename,
                maybe_deprecated = if command.deprecated {
                    DEPRECATED_ATTRIBUTE
                } else {
                    ""
                },
                bin = plugin.binary,
                space_if_args = if command.args.is_empty() { "" } else { " " },
                args = command.args.join(" ")
            ));

            format_snippet_values.insert(format!(
                "{INDENT}{INDENT}{INDENT}Self::{enum_value} => (&{module_name}::COMMANDS, {module_name}::set_args, {module_name}::IS_STDIN),",
             ));

            as_ref_values.insert(format!(
                "{INDENT}{INDENT}{INDENT}Self::{} => \"{}\",",
                command.enum_value, command.serde_rename,
            ));

            all_commands.push(command);
        }
    }

    std::fs::write(
        "mdsf/tests/tooling.rs",
        format!(
            "{GENERATED_FILE_COMMENT}

#[allow(dead_code)]
mod common;

{}
",
            all_tests.join("\n")
        ),
    )
    .unwrap();

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

    let reversible_enum_test = generate_reversible_tooling_test(enum_value_names);

    let mod_file_contents = format!(
        "{GENERATED_FILE_COMMENT}

{}

#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
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
{INDENT}{INDENT}debug_enabled: bool,
{INDENT}{INDENT}config_runners: &crate::config::MdsfConfigRunners,
{INDENT}) -> Result<(bool, Option<String>), crate::error::MdsfError> {{
{INDENT}{INDENT}let (commands, set_args_fn, is_stdin): (
{INDENT}{INDENT}{INDENT}&[crate::runners::CommandType],
{INDENT}{INDENT}{INDENT}crate::execution::SetArgsFn,
{INDENT}{INDENT}{INDENT}bool,
{INDENT}{INDENT}) = match self {{
{}
{INDENT}{INDENT}}};

{INDENT}{INDENT}crate::execution::run_tools(
{INDENT}{INDENT}{INDENT}commands,
{INDENT}{INDENT}{INDENT}snippet_path,
{INDENT}{INDENT}{INDENT}set_args_fn,
{INDENT}{INDENT}{INDENT}timeout,
{INDENT}{INDENT}{INDENT}is_stdin,
{INDENT}{INDENT}{INDENT}debug_enabled,
{INDENT}{INDENT}{INDENT}config_runners,
{INDENT}{INDENT})
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

{reversible_enum_test}",
        modules.join("\n"),
        tooling_enum_content.join("\n\n"),
        format_snippet_content.join("\n"),
        as_ref_content.join("\n"),
    );

    std::fs::write(format!("{folder}/mod.rs"), mod_file_contents)?;

    Ok(all_commands)
}
