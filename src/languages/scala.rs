use schemars::JsonSchema;

use crate::formatters::{scalafmt::format_using_scalafmt, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Scalafmt => format_using_scalafmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_blade {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Scala;

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
        .format(snippet_path)
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
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
