use json_comments::{CommentSettings, StripComments};
use schemars::JsonSchema;

use crate::{
    error::MdsfError,
    languages::{
        blade::Blade, c::C, cabal::Cabal, clojure::Clojure, cpp::Cpp, crystal::Crystal,
        csharp::CSharp, css::Css, d::D, dart::Dart, elixir::Elixir, elm::Elm, erb::Erb,
        erlang::Erlang, fortran::Fortran, fsharp::FSharp, gleam::Gleam, go::Go, graphql::GraphQL,
        groovy::Groovy, handlebars::Handlebars, haskell::Haskell, hcl::Hcl, html::Html, java::Java,
        javascript::JavaScript, json::Json, julia::Julia, just::Just, kcl::Kcl, kotlin::Kotlin,
        lua::Lua, markdown::Markdown, mustache::Mustache, nim::Nim, nix::Nix, nunjucks::Nunjucks,
        objective_c::ObjectiveC, ocaml::OCaml, perl::Perl, protobuf::Protobuf, puppet::Puppet,
        purescript::PureScript, python::Python, rescript::ReScript,
        restructuredtext::ReStructuredText, roc::Roc, ruby::Ruby, rust::Rust, scala::Scala,
        shell::Shell, solidity::Solidity, sql::Sql, swift::Swift, toml::Toml,
        typescript::TypeScript, vue::Vue, xml::Xml, yaml::Yaml, zig::Zig, Lang,
    },
    runners::JavaScriptRuntime,
    terminal::print_config_not_found,
};

#[derive(serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq,))]
#[schemars(deny_unknown_fields)]
pub struct MdsfConfig {
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
    pub d: Lang<D>,

    #[serde(default)]
    pub dart: Lang<Dart>,

    #[serde(default)]
    pub elixir: Lang<Elixir>,

    #[serde(default)]
    pub elm: Lang<Elm>,

    #[serde(default)]
    pub erb: Lang<Erb>,

    #[serde(default)]
    pub erlang: Lang<Erlang>,

    #[serde(default)]
    pub fortran: Lang<Fortran>,

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
    pub handlebars: Lang<Handlebars>,

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
    pub kcl: Lang<Kcl>,

    #[serde(default)]
    pub kotlin: Lang<Kotlin>,

    #[serde(default)]
    pub lua: Lang<Lua>,

    #[serde(default)]
    pub markdown: Lang<Markdown>,

    #[serde(default)]
    pub nim: Lang<Nim>,

    #[serde(default)]
    pub nix: Lang<Nix>,

    #[serde(default)]
    pub objective_c: Lang<ObjectiveC>,

    #[serde(default)]
    pub ocaml: Lang<OCaml>,

    #[serde(default)]
    pub perl: Lang<Perl>,

    #[serde(default)]
    pub protobuf: Lang<Protobuf>,

    #[serde(default)]
    pub purescript: Lang<PureScript>,

    #[serde(default)]
    pub python: Lang<Python>,

    #[serde(default)]
    pub rescript: Lang<ReScript>,

    #[serde(default)]
    pub restructuredtext: Lang<ReStructuredText>,

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
    pub solidity: Lang<Solidity>,

    #[serde(default)]
    pub sql: Lang<Sql>,

    #[serde(default)]
    pub swift: Lang<Swift>,

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

    #[serde(default)]
    pub nunjucks: Lang<Nunjucks>,

    #[serde(default)]
    pub mustache: Lang<Mustache>,

    #[serde(default)]
    pub puppet: Lang<Puppet>,
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
            d: Lang::<D>::default(),
            dart: Lang::<Dart>::default(),
            elixir: Lang::<Elixir>::default(),
            elm: Lang::<Elm>::default(),
            erb: Lang::<Erb>::default(),
            erlang: Lang::<Erlang>::default(),
            fortran: Lang::<Fortran>::default(),
            fsharp: Lang::<FSharp>::default(),
            gleam: Lang::<Gleam>::default(),
            go: Lang::<Go>::default(),
            graphql: Lang::<GraphQL>::default(),
            groovy: Lang::<Groovy>::default(),
            handlebars: Lang::<_>::default(),
            haskell: Lang::<Haskell>::default(),
            hcl: Lang::<Hcl>::default(),
            html: Lang::<Html>::default(),
            java: Lang::<Java>::default(),
            javascript: Lang::<JavaScript>::default(),
            json: Lang::<Json>::default(),
            julia: Lang::<Julia>::default(),
            just: Lang::<Just>::default(),
            kcl: Lang::<Kcl>::default(),
            kotlin: Lang::<Kotlin>::default(),
            lua: Lang::<Lua>::default(),
            markdown: Lang::<Markdown>::default(),
            nim: Lang::<Nim>::default(),
            nix: Lang::<Nix>::default(),
            objective_c: Lang::<ObjectiveC>::default(),
            ocaml: Lang::<OCaml>::default(),
            perl: Lang::<Perl>::default(),
            protobuf: Lang::<Protobuf>::default(),
            purescript: Lang::<PureScript>::default(),
            python: Lang::<Python>::default(),
            rescript: Lang::<ReScript>::default(),
            restructuredtext: Lang::<ReStructuredText>::default(),
            roc: Lang::<Roc>::default(),
            ruby: Lang::<Ruby>::default(),
            rust: Lang::<Rust>::default(),
            scala: Lang::<Scala>::default(),
            shell: Lang::<Shell>::default(),
            sql: Lang::<Sql>::default(),
            solidity: Lang::<Solidity>::default(),
            swift: Lang::<Swift>::default(),
            toml: Lang::<Toml>::default(),
            typescript: Lang::<TypeScript>::default(),
            vue: Lang::<Vue>::default(),
            xml: Lang::<Xml>::default(),
            yaml: Lang::<Yaml>::default(),
            zig: Lang::<Zig>::default(),
            mustache: Lang::<Mustache>::default(),
            nunjucks: Lang::<Nunjucks>::default(),
            puppet: Lang::<Puppet>::default(),
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
                Self::parse(&raw_config).map_err(|_serde_error| MdsfError::ConfigParse(path))
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    print_config_not_found();
                    Ok(Self::default())
                } else {
                    Err(MdsfError::from(error))
                }
            }
        }
    }

    #[inline]
    pub fn parse(input: &str) -> serde_json::Result<Self> {
        let stripped = StripComments::with_settings(CommentSettings::c_style(), input.as_bytes());

        serde_json::from_reader(stripped)
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

    #[test]
    fn it_should_ignore_comments() {
        let r = r#"{
    // this is a slash comment
    "javascript": {
        "enabled": false
    },
    "rust": {
        "enabled": false
    },
    /* this is a multiline comment
    "roc": {
        "enabled": false
    }
    */
    "go": {
        "enabled": false // hello world
    }
}"#;

        let c = MdsfConfig::parse(r).expect("it to parse the config");

        assert!(!c.javascript.enabled);

        assert!(!c.rust.enabled);

        assert!(c.roc.enabled);

        assert!(!c.go.enabled);
    }
}
