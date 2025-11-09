use crate::{config::MdsfTool, execution::MdsfFormatter};

mod c;
mod cpp;
mod csharp;
mod go;
mod java;
mod javascript;
mod json;
mod python;
mod ruby;
mod rust;
mod shell;
mod sql;
mod toml;
mod typescript;
mod yaml;

#[inline]
pub fn default_tools() -> std::collections::BTreeMap<String, MdsfFormatter<MdsfTool>> {
    std::collections::BTreeMap::from_iter([
        c::default_config(),
        cpp::default_config(),
        csharp::default_config(),
        go::default_config(),
        java::default_config(),
        javascript::default_config(),
        json::default_config(),
        python::default_config(),
        ruby::default_config(),
        rust::default_config(),
        shell::default_config(),
        sql::default_config(),
        toml::default_config(),
        typescript::default_config(),
        yaml::default_config(),
    ])
}
