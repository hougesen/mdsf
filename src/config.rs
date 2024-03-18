use schemars::JsonSchema;

use crate::languages::{
    blade::Blade, c::C, cpp::Cpp, crystal::Crystal, csharp::CSharp, css::Css, dart::Dart,
    elixir::Elixir, elm::Elm, gleam::Gleam, go::Go, graphql::GraphQL, html::Html, java::Java,
    javascript::JavaScript, json::Json, just::Just, lua::Lua, markdown::Markdown, nim::Nim,
    objective_c::ObjectiveC, ocaml::OCaml, protobuf::Protobuf, python::Python, rescript::ReScript,
    roc::Roc, ruby::Ruby, rust::Rust, shell::Shell, sql::Sql, toml::Toml, typescript::TypeScript,
    vue::Vue, xml::Xml, yaml::Yaml, zig::Zig, Lang,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct MdsfConfig {
    #[schemars(skip)]
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    #[serde(default)]
    pub blade: Lang<Blade>,

    #[serde(default)]
    pub c: Lang<C>,

    #[serde(default)]
    pub cpp: Lang<Cpp>,

    #[serde(default)]
    pub crystal: Lang<Crystal>,

    #[serde(default)]
    pub csharp: Lang<CSharp>,

    #[serde(default)]
    pub css: Lang<Css>,

    #[serde(default)]
    pub dart: Lang<Dart>,

    #[serde(default)]
    pub elixir: Lang<Elixir>,

    #[serde(default)]
    pub elm: Lang<Elm>,

    #[serde(default)]
    pub gleam: Lang<Gleam>,

    #[serde(default)]
    pub go: Lang<Go>,

    #[serde(default)]
    pub graphql: Lang<GraphQL>,

    #[serde(default)]
    pub html: Lang<Html>,

    #[serde(default)]
    pub java: Lang<Java>,

    #[serde(default)]
    pub javascript: Lang<JavaScript>,

    #[serde(default)]
    pub json: Lang<Json>,

    #[serde(default)]
    pub just: Lang<Just>,

    #[serde(default)]
    pub lua: Lang<Lua>,

    #[serde(default)]
    pub markdown: Lang<Markdown>,

    #[serde(default)]
    pub nim: Lang<Nim>,

    #[serde(default)]
    pub objective_c: Lang<ObjectiveC>,

    #[serde(default)]
    pub ocaml: Lang<OCaml>,

    #[serde(default)]
    pub protobuf: Lang<Protobuf>,

    #[serde(default)]
    pub python: Lang<Python>,

    #[serde(default)]
    pub rescript: Lang<ReScript>,

    #[serde(default)]
    pub roc: Lang<Roc>,

    #[serde(default)]
    pub ruby: Lang<Ruby>,

    #[serde(default)]
    pub rust: Lang<Rust>,

    #[serde(default)]
    pub shell: Lang<Shell>,

    #[serde(default)]
    pub sql: Lang<Sql>,

    #[serde(default)]
    pub toml: Lang<Toml>,

    #[serde(default)]
    pub typescript: Lang<TypeScript>,

    #[serde(default)]
    pub vue: Lang<Vue>,

    #[serde(default)]
    pub xml: Lang<Xml>,

    #[serde(default)]
    pub yaml: Lang<Yaml>,

    #[serde(default)]
    pub zig: Lang<Zig>,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),

            blade: Lang::<Blade>::default(),
            c: Lang::<C>::default(),
            cpp: Lang::<Cpp>::default(),
            crystal: Lang::<Crystal>::default(),
            csharp: Lang::<CSharp>::default(),
            css: Lang::<Css>::default(),
            dart: Lang::<Dart>::default(),
            elm: Lang::<Elm>::default(),
            elixir: Lang::<Elixir>::default(),
            gleam: Lang::<Gleam>::default(),
            go: Lang::<Go>::default(),
            graphql: Lang::<GraphQL>::default(),
            html: Lang::<Html>::default(),
            java: Lang::<Java>::default(),
            javascript: Lang::<JavaScript>::default(),
            json: Lang::<Json>::default(),
            just: Lang::<Just>::default(),
            lua: Lang::<Lua>::default(),
            markdown: Lang::<Markdown>::default(),
            nim: Lang::<Nim>::default(),
            objective_c: Lang::<ObjectiveC>::default(),
            ocaml: Lang::<OCaml>::default(),
            protobuf: Lang::<Protobuf>::default(),
            python: Lang::<Python>::default(),
            rescript: Lang::<ReScript>::default(),
            roc: Lang::<Roc>::default(),
            ruby: Lang::<Ruby>::default(),
            rust: Lang::<Rust>::default(),
            shell: Lang::<Shell>::default(),
            sql: Lang::<Sql>::default(),
            toml: Lang::<Toml>::default(),
            typescript: Lang::<TypeScript>::default(),
            vue: Lang::<Vue>::default(),
            xml: Lang::<Xml>::default(),
            yaml: Lang::<Yaml>::default(),
            zig: Lang::<Zig>::default(),
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
fn default_schema_location() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!(
        "https://raw.githubusercontent.com/hougesen/mdsf/main/schemas/v{package_version}/mdsf.schema.json"
    )
}

#[cfg(test)]
mod test_config {
    use super::MdsfConfig;

    #[test]
    fn schema_should_be_serializable() {
        let config = MdsfConfig::default();

        let json = serde_json::to_string_pretty(&config).expect("it to be serializable");

        let loaded = serde_json::from_str::<MdsfConfig>(&json).expect("it to be parsed");

        assert_eq!(config, loaded);
    }

    #[test]
    fn json_schema_should_be_serializable() {
        serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))
            .expect("it to be serializable");
    }
}
