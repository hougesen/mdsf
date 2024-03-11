use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::stylua::format_using_stylua};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LuaFormatter {
    #[default]
    #[serde(rename = "stylua")]
    Stylua,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Lua {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: LuaFormatter,
}

impl Default for Lua {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: LuaFormatter::default(),
        }
    }
}

impl LanguageFormatter for Lua {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            LuaFormatter::Stylua => format_using_stylua(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Lua, LuaFormatter};

    const INPUT: &str = "

        local               function        add (                                       a , b
)

return              a +b


end

    ";

    const EXTENSION: &str = crate::languages::Language::Lua.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Lua::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lua {
            enabled: false,
            formatter: LuaFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}
