use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{cabal_format::format_using_cabal_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Cabal {
    #[default]
    #[serde(rename = "cabal_format")]
    CabalFormat,
}

impl Default for Lang<Cabal> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Cabal>::default(),
        }
    }
}

impl Default for MdsfFormatter<Cabal> {
    #[inline]
    fn default() -> Self {
        Self::Single(Cabal::CabalFormat)
    }
}

impl LanguageFormatter for Cabal {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::CabalFormat => format_using_cabal_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Cabal {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CabalFormat => write!(f, "cabal_format"),
        }
    }
}

#[cfg(test)]
mod test_cabal {
    use super::Cabal;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT:&str = "cabal-version: 2.4
name: mdsf
version: 0

executable msdf
    default-language: Haskell2010
    hs-source-dirs: src
    main-is: Mdsf.hs
    build-depends: base >=4.11 && <4.13, pretty >=1.1.3.6 && <1.2, bytestring, Cabal ^>=2.5, containers ^>=0.5.11.0 || ^>=0.6.0.1
    other-extensions:
      DeriveFunctor FlexibleContexts ExistentialQuantification OverloadedStrings
      RankNTypes";

    const EXTENSION: &str = crate::languages::Language::Toml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Cabal>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Cabal> {
            enabled: false,
            formatter: MdsfFormatter::Single(Cabal::CabalFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_cabal_format() {
        let l = Lang::<Cabal> {
            enabled: true,
            formatter: MdsfFormatter::Single(Cabal::CabalFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "cabal-version: 2.4
name:          mdsf
version:       0

executable msdf
    main-is:          Mdsf.hs
    hs-source-dirs:   src
    default-language: Haskell2010
    other-extensions:
        DeriveFunctor FlexibleContexts ExistentialQuantification
        OverloadedStrings RankNTypes

    build-depends:
        base >=4.11 && <4.13,
        pretty >=1.1.3.6 && <1.2,
        bytestring,
        Cabal ^>=2.5,
        containers ^>=0.5.11.0 || ^>=0.6.0.1
";

        assert_eq!(output, expected_output);
    }
}
