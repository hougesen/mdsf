use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{clang_format::format_using_clang_format, format_multiple, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ObjectiveCFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct ObjectiveC {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<ObjectiveCFormatter>,
}

impl Default for ObjectiveC {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ObjectiveCFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<ObjectiveCFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(ObjectiveCFormatter::ClangFormat)
    }
}

impl LanguageFormatter<ObjectiveCFormatter> for ObjectiveC {
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
        formatter: &ObjectiveCFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            ObjectiveCFormatter::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_objective_c {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{ObjectiveC, ObjectiveCFormatter};

    const INPUT: &str = "int add(int a,int b){
            a - a ;
       return a + b;
    }";

    const EXTENSION: &str = crate::languages::Language::ObjectiveC.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(ObjectiveC::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(ObjectiveC {
            enabled: false,
            formatter: MdsfFormatter::Single(ObjectiveCFormatter::ClangFormat),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let expected_output = "int add(int a, int b) {
  a - a;
  return a + b;
}";

        let l = ObjectiveC {
            enabled: true,
            formatter: MdsfFormatter::Single(ObjectiveCFormatter::ClangFormat),
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
