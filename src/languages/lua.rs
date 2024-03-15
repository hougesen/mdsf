use schemars::JsonSchema;

use crate::formatters::{stylua::format_using_stylua, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Lua {
    #[default]
    #[serde(rename = "stylua")]
    Stylua,
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
        Self::Single(Lua::Stylua)
    }
}

impl LanguageFormatter for Lua {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Stylua => format_using_stylua(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_lua {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Lua;

    const INPUT: &str = "

        local               function        add (                                       a , b
)

return              a +b


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
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_stylua() {
        let expected_output = "local function add(a, b)\n\treturn a + b\nend\n";

        let l = Lang::<Lua> {
            enabled: true,
            formatter: MdsfFormatter::Single(Lua::Stylua),
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
