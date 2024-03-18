use schemars::JsonSchema;

use crate::formatters::{crystal_format::format_using_crystal_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Crystal {
    #[default]
    #[serde(rename = "crystal_format")]
    CrystalFormat,
}

impl Default for Lang<Crystal> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Crystal>::default(),
        }
    }
}

impl Default for MdsfFormatter<Crystal> {
    #[inline]
    fn default() -> Self {
        Self::Single(Crystal::CrystalFormat)
    }
}

impl LanguageFormatter for Crystal {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::CrystalFormat => format_using_crystal_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_crystal {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Crystal;

    const INPUT: &str = "def add(a, b)  return a + b end";

    const EXTENSION: &str = crate::languages::Language::Crystal.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Crystal>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Crystal> {
            enabled: false,
            formatter: MdsfFormatter::Single(Crystal::CrystalFormat),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(crystal)]
    #[test]
    fn test_crystal_format() {
        let expected_output = "def add(a, b)
  return a + b
end
";
        let l = Lang::<Crystal> {
            enabled: true,
            formatter: MdsfFormatter::Single(Crystal::CrystalFormat),
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
