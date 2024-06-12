use crate::formatters::{MdsfFormatter, Tooling};

mod c;
mod cpp;
mod go;
mod haskell;
mod java;
mod javascript;
mod json;
mod python;
mod ruby;
mod rust;
mod sql;
mod swift;
mod toml;
mod typescript;
mod yaml;
mod zig;

#[inline]
pub fn default_tools() -> std::collections::HashMap<String, MdsfFormatter<Tooling>> {
    std::collections::HashMap::from_iter([
        c::default_config(),
        cpp::default_config(),
        java::default_config(),
        go::default_config(),
        javascript::default_config(),
        json::default_config(),
        python::default_config(),
        sql::default_config(),
        ruby::default_config(),
        rust::default_config(),
        swift::default_config(),
        haskell::default_config(),
        zig::default_config(),
        toml::default_config(),
        typescript::default_config(),
        zig::default_config(),
        yaml::default_config(),
    ])
}
