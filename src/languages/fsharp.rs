use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{fantomas::format_using_fantomas, MdsfFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum FSharp {
    #[default]
    #[serde(rename = "fantomas")]
    Fantomas,
}

impl Default for Lang<FSharp> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<FSharp>::default(),
        }
    }
}

impl Default for MdsfFormatter<FSharp> {
    #[inline]
    fn default() -> Self {
        Self::Single(FSharp::Fantomas)
    }
}

impl LanguageFormatter for FSharp {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Fantomas => format_using_fantomas(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_fsharp {
    use super::FSharp;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    const INPUT: &str = "
let add  a b   =  
                                                      a +  b
            ";
    const EXTENSION: &str = crate::languages::Language::FSharp.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<FSharp>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<FSharp> {
            enabled: false,
            formatter: MdsfFormatter::<FSharp>::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(fantomas)]
    #[test]
    fn test_fantomas() {
        let l = Lang::<FSharp> {
            enabled: true,
            formatter: MdsfFormatter::Single(FSharp::Fantomas),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "let add a b = a + b
";
        assert_eq!(output, expected_output);
    }
}
