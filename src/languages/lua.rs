use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        luaformatter::format_using_luaformatter, stylua::format_using_stylua, MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Lua {
    #[default]
    #[serde(rename = "stylua")]
    Stylua,
    #[serde(rename = "luaformatter")]
    LuaFormatter,
}

impl Default for Lang<Lua> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Lua>::default(),
        }
    }
}

impl Default for MdsfFormatter<Lua> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Lua::Stylua),
            Self::Single(Lua::LuaFormatter),
        ])])
    }
}

impl LanguageFormatter for Lua {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Stylua => format_using_stylua(snippet_path),
            Self::LuaFormatter => format_using_luaformatter(snippet_path),
        }
    }
}

impl core::fmt::Display for Lua {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stylua => write!(f, "stylua"),
            Self::LuaFormatter => write!(f, "luaformatter"),
        }
    }
}

#[cfg(test)]
mod test_lua {
    use super::Lua;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "

        local               function        add (                                       a , b
)
    local c =  a  + b
return              c


end
    ";

    const EXTENSION: &str = crate::languages::Language::Lua.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Lua>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Lua> {
            enabled: false,
            formatter: MdsfFormatter::Single(Lua::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_stylua() {
        let expected_output = "local function add(a, b)
\tlocal c = a + b
\treturn c
end
";

        let l = Lang::<Lua> {
            enabled: true,
            formatter: MdsfFormatter::Single(Lua::Stylua),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(lua-format)]
    #[test]
    fn test_luaformatter() {
        let expected_output = "local function add(a, b)
    local c = a + b
    return c

end
";

        let l = Lang::<Lua> {
            enabled: true,
            formatter: MdsfFormatter::Single(Lua::LuaFormatter),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
