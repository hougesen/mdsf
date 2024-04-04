use schemars::JsonSchema;

use crate::{
    error::MdsfError,
    languages::{
        blade::Blade, c::C, cabal::Cabal, clojure::Clojure, cpp::Cpp, crystal::Crystal,
        csharp::CSharp, css::Css, dart::Dart, elixir::Elixir, elm::Elm, erlang::Erlang,
        fsharp::FSharp, gleam::Gleam, go::Go, graphql::GraphQL, groovy::Groovy, haskell::Haskell,
        hcl::Hcl, html::Html, java::Java, javascript::JavaScript, json::Json, julia::Julia,
        just::Just, kotlin::Kotlin, lua::Lua, markdown::Markdown, nim::Nim,
        objective_c::ObjectiveC, ocaml::OCaml, perl::Perl, protobuf::Protobuf,
        purescript::PureScript, python::Python, rescript::ReScript, roc::Roc, ruby::Ruby,
        rust::Rust, scala::Scala, shell::Shell, sql::Sql, swift::Swift, toml::Toml,
        typescript::TypeScript, vue::Vue, xml::Xml, yaml::Yaml, zig::Zig, Lang,
    },
    runners::JavaScriptRuntime,
    terminal::print_config_info,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct MdsfConfig {
    #[schemars(skip)]
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    /// Format the processed document with the selected markdown formatter.
    #[serde(default)]
    pub format_finished_document: bool,

    /// Set npm script runner runtime.
    ///
    /// Should be considered experimental since not all tools support being run using Bun/Deno.
    ///
    /// `node -> npx`
    ///
    /// `bun -> bunx`
    ///
    /// `deno -> deno run`
    #[serde(default)]
    pub javascript_runtime: JavaScriptRuntime,

    #[serde(default)]
    pub blade: Lang<Blade>,

    #[serde(default)]
    pub c: Lang<C>,

    #[serde(default)]
    pub cabal: Lang<Cabal>,

    #[serde(default)]
    pub clojure: Lang<Clojure>,

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
    pub erlang: Lang<Erlang>,

    #[serde(default)]
    pub fsharp: Lang<FSharp>,

    #[serde(default)]
    pub gleam: Lang<Gleam>,

    #[serde(default)]
    pub go: Lang<Go>,

    #[serde(default)]
    pub graphql: Lang<GraphQL>,

    #[serde(default)]
    pub groovy: Lang<Groovy>,

    #[serde(default)]
    pub haskell: Lang<Haskell>,

    #[serde(default)]
    pub hcl: Lang<Hcl>,

    #[serde(default)]
    pub html: Lang<Html>,

    #[serde(default)]
    pub java: Lang<Java>,

    #[serde(default)]
    pub javascript: Lang<JavaScript>,

    #[serde(default)]
    pub json: Lang<Json>,

    #[serde(default)]
    pub julia: Lang<Julia>,

    #[serde(default)]
    pub just: Lang<Just>,

    #[serde(default)]
    pub kotlin: Lang<Kotlin>,

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
    pub perl: Lang<Perl>,

    #[serde(default)]
    pub purescript: Lang<PureScript>,

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
    pub scala: Lang<Scala>,

    #[serde(default)]
    pub shell: Lang<Shell>,

    #[serde(default)]
    pub swift: Lang<Swift>,

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
            format_finished_document: false,
            javascript_runtime: JavaScriptRuntime::default(),

            blade: Lang::<Blade>::default(),
            c: Lang::<C>::default(),
            cabal: Lang::<Cabal>::default(),
            clojure: Lang::<Clojure>::default(),
            cpp: Lang::<Cpp>::default(),
            crystal: Lang::<Crystal>::default(),
            csharp: Lang::<CSharp>::default(),
            css: Lang::<Css>::default(),
            dart: Lang::<Dart>::default(),
            elixir: Lang::<Elixir>::default(),
            elm: Lang::<Elm>::default(),
            erlang: Lang::<Erlang>::default(),
            fsharp: Lang::<FSharp>::default(),
            gleam: Lang::<Gleam>::default(),
            go: Lang::<Go>::default(),
            graphql: Lang::<GraphQL>::default(),
            groovy: Lang::<Groovy>::default(),
            haskell: Lang::<Haskell>::default(),
            hcl: Lang::<Hcl>::default(),
            html: Lang::<Html>::default(),
            java: Lang::<Java>::default(),
            javascript: Lang::<JavaScript>::default(),
            json: Lang::<Json>::default(),
            julia: Lang::<Julia>::default(),
            just: Lang::<Just>::default(),
            kotlin: Lang::<Kotlin>::default(),
            lua: Lang::<Lua>::default(),
            markdown: Lang::<Markdown>::default(),
            nim: Lang::<Nim>::default(),
            objective_c: Lang::<ObjectiveC>::default(),
            ocaml: Lang::<OCaml>::default(),
            protobuf: Lang::<Protobuf>::default(),
            perl: Lang::<Perl>::default(),
            python: Lang::<Python>::default(),
            purescript: Lang::<PureScript>::default(),
            rescript: Lang::<ReScript>::default(),
            roc: Lang::<Roc>::default(),
            ruby: Lang::<Ruby>::default(),
            rust: Lang::<Rust>::default(),
            scala: Lang::<Scala>::default(),
            shell: Lang::<Shell>::default(),
            swift: Lang::<Swift>::default(),
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
    pub fn load() -> Result<Self, MdsfError> {
        let dir = std::env::current_dir()?;

        let path = dir.join("mdsf.json");

        match std::fs::read_to_string(&path) {
            Ok(raw_config) => {
                if let Ok(config) = serde_json::from_str::<Self>(&raw_config) {
                    print_config_info(Some(&path));
                    Ok(config)
                } else {
                    Err(MdsfError::ConfigParse(path))
                }
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    print_config_info(None);
                    Ok(Self::default())
                }

                _ => Err(MdsfError::from(error)),
            },
        }
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
