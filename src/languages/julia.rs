use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{juliaformatter_jl::format_using_juliaformatter_jl, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Julia {
    #[default]
    #[serde(rename = "juliaformatter.jl")]
    JuliaFormatterJl,
}

impl Default for Lang<Julia> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Julia>::default(),
        }
    }
}

impl Default for MdsfFormatter<Julia> {
    #[inline]
    fn default() -> Self {
        Self::Single(Julia::JuliaFormatterJl)
    }
}

impl LanguageFormatter for Julia {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::JuliaFormatterJl => format_using_juliaformatter_jl(snippet_path),
        }
    }
}

impl core::fmt::Display for Julia {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JuliaFormatterJl => write!(f, "juliaformatter.jl"),
        }
    }
}

#[cfg(test)]
mod test_julia {
    use super::Julia;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end ";

    const EXTENSION: &str = crate::languages::Language::Julia.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Julia>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Julia> {
            enabled: false,
            formatter: MdsfFormatter::Single(Julia::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(julia)]
    #[test]
    fn test_juliaformatter_jl() {
        let l = Lang::<Julia> {
            enabled: true,
            formatter: MdsfFormatter::Single(Julia::JuliaFormatterJl),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "function add(a::Int32, b::Int32)
    c = a + b
    return c
end
";

        assert_eq!(output, expected_output);
    }
}
