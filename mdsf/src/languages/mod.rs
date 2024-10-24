use crate::{execution::MdsfFormatter, tools::Tooling};

mod c;
mod clojure;
mod cpp;
mod crystal;
mod csharp;
mod css;
mod d;
mod dart;
mod elixir;
mod elm;
mod erlang;
mod gleam;
mod go;
mod haskell;
mod html;
mod java;
mod javascript;
mod json;
mod kotlin;
mod lua;
mod nim;
mod ocaml;
mod python;
mod roc;
mod ruby;
mod rust;
mod shell;
mod sql;
mod swift;
mod toml;
mod typescript;
mod yaml;
mod zig;

#[inline]
pub fn default_tools() -> std::collections::BTreeMap<String, MdsfFormatter<Tooling>> {
    std::collections::BTreeMap::from_iter([
        c::default_config(),
        clojure::default_config(),
        cpp::default_config(),
        crystal::default_config(),
        csharp::default_config(),
        css::default_config(),
        d::default_config(),
        dart::default_config(),
        elixir::default_config(),
        elm::default_config(),
        erlang::default_config(),
        gleam::default_config(),
        go::default_config(),
        haskell::default_config(),
        html::default_config(),
        java::default_config(),
        javascript::default_config(),
        json::default_config(),
        kotlin::default_config(),
        lua::default_config(),
        nim::default_config(),
        ocaml::default_config(),
        python::default_config(),
        roc::default_config(),
        ruby::default_config(),
        rust::default_config(),
        shell::default_config(),
        sql::default_config(),
        swift::default_config(),
        toml::default_config(),
        typescript::default_config(),
        yaml::default_config(),
        zig::default_config(),
    ])
}
