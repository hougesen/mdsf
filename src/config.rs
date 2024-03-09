use schemars::JsonSchema;

use crate::languages::{
    c::C, cpp::Cpp, csharp::CSharp, css::Css, dart::Dart, elixir::Elixir, gleam::Gleam, go::Go,
    html::Html, java::Java, javascript::JavaScript, json::Json, lua::Lua, markdown::Markdown,
    nim::Nim, objective_c::ObjectiveC, protobuf::Protobuf, python::Python, ruby::Ruby, rust::Rust,
    shell::Shell, sql::Sql, toml::Toml, typescript::TypeScript, vue::Vue, yaml::Yaml, zig::Zig,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct MdsfConfig {
    #[schemars(skip)]
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    #[serde(default)]
    pub c: C,

    #[serde(default)]
    pub cpp: Cpp,

    #[serde(default)]
    pub csharp: CSharp,

    #[serde(default)]
    pub css: Css,

    #[serde(default)]
    pub dart: Dart,

    #[serde(default)]
    pub elixir: Elixir,

    #[serde(default)]
    pub gleam: Gleam,

    #[serde(default)]
    pub go: Go,

    #[serde(default)]
    pub html: Html,

    #[serde(default)]
    pub java: Java,

    #[serde(default)]
    pub javascript: JavaScript,

    #[serde(default)]
    pub json: Json,

    #[serde(default)]
    pub lua: Lua,

    #[serde(default)]
    pub markdown: Markdown,

    #[serde(default)]
    pub nim: Nim,

    #[serde(default)]
    pub objective_c: ObjectiveC,

    #[serde(default)]
    pub protobuf: Protobuf,

    #[serde(default)]
    pub python: Python,

    #[serde(default)]
    pub ruby: Ruby,

    #[serde(default)]
    pub rust: Rust,

    #[serde(default)]
    pub shell: Shell,

    #[serde(default)]
    pub sql: Sql,

    #[serde(default)]
    pub toml: Toml,

    #[serde(default)]
    pub typescript: TypeScript,

    #[serde(default)]
    pub vue: Vue,

    #[serde(default)]
    pub yaml: Yaml,

    #[serde(default)]
    pub zig: Zig,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),

            c: C::default(),
            cpp: Cpp::default(),
            csharp: CSharp::default(),
            css: Css::default(),
            dart: Dart::default(),
            elixir: Elixir::default(),
            gleam: Gleam::default(),
            go: Go::default(),
            html: Html::default(),
            java: Java::default(),
            javascript: JavaScript::default(),
            json: Json::default(),
            lua: Lua::default(),
            markdown: Markdown::default(),
            nim: Nim::default(),
            objective_c: ObjectiveC::default(),
            protobuf: Protobuf::default(),
            python: Python::default(),
            ruby: Ruby::default(),
            rust: Rust::default(),
            shell: Shell::default(),
            sql: Sql::default(),
            toml: Toml::default(),
            typescript: TypeScript::default(),
            vue: Vue::default(),
            yaml: Yaml::default(),
            zig: Zig::default(),
        }
    }
}

impl MdsfConfig {
    #[inline]
    pub fn load() -> Self {
        if let Ok(raw_config) = std::fs::read_to_string("mdsf.json") {
            if let Ok(config) = serde_json::from_str::<Self>(&raw_config) {
                return config;
            }
        }

        Self::default()
    }
}

#[inline]
pub const fn default_enabled() -> bool {
    true
}

#[inline]
fn default_schema_location() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!(
        "https://raw.githubusercontent.com/hougesen/mdsf/main/schemas/v{package_version}/mdsf.schema.json"
    )
}
