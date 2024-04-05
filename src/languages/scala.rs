use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{scalafmt::format_using_scalafmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Scala {
    #[default]
    #[serde(rename = "scalafmt")]
    Scalafmt,
}

impl Default for Lang<Scala> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Scala>::default(),
        }
    }
}

impl Default for MdsfFormatter<Scala> {
    #[inline]
    fn default() -> Self {
        Self::Single(Scala::Scalafmt)
    }
}

impl LanguageFormatter for Scala {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Scalafmt => format_using_scalafmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Scala {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Scalafmt => write!(f, "scalafmt"),
        }
    }
}

#[cfg(test)]
mod test_scala {
    use super::Scala;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = r#"
         object A    {  println   ("HELLO!"  )  }


// comment
 "#;

    const EXTENSION: &str = crate::languages::Language::Blade.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Scala>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Scala> {
            enabled: false,
            formatter: MdsfFormatter::Single(Scala::Scalafmt),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(scalafmt)]
    #[test]
    fn test_scalafmt() {
        let expected_output = r#"object A{ println("HELLO!") }

// comment
"#;

        let l = Lang::<Scala> {
            enabled: true,
            formatter: MdsfFormatter::Single(Scala::Scalafmt),
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
