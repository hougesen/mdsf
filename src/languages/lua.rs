use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, stylua::format_using_stylua, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum LuaFormatter {
    #[default]
    #[serde(rename = "stylua")]
    Stylua,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Lua {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<LuaFormatter>,
}

impl Default for Lua {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<LuaFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<LuaFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(LuaFormatter::Stylua)
    }
}

impl LanguageFormatter<LuaFormatter> for Lua {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &LuaFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            LuaFormatter::Stylua => format_using_stylua(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_lua {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

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
        assert!(Lua::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lua {
            enabled: false,
            formatter: MdsfFormatter::Single(LuaFormatter::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_stylua() {
        let expected_output = "local function add(a, b)\n\treturn a + b\nend\n";

        let l = Lua {
            enabled: true,
            formatter: MdsfFormatter::Single(LuaFormatter::Stylua),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
