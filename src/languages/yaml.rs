use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        prettier::format_using_prettier, yamlfix::format_using_yamlfix,
        yamlfmt::format_using_yamlfmt, MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Yaml {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "yamlfmt")]
    YamlFmt,
    #[serde(rename = "yamlfix")]
    YamlFix,
}

impl Default for Lang<Yaml> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Yaml>::default(),
        }
    }
}

impl Default for MdsfFormatter<Yaml> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Yaml::Prettier),
            Self::Single(Yaml::YamlFmt),
            Self::Single(Yaml::YamlFix),
        ])])
    }
}

impl LanguageFormatter for Yaml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path),
            Self::YamlFmt => format_using_yamlfmt(snippet_path),
            Self::YamlFix => format_using_yamlfix(snippet_path),
        }
    }
}

impl core::fmt::Display for Yaml {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::YamlFmt => write!(f, "yamlfmt"),
            Self::YamlFix => write!(f, "yamlfix"),
        }
    }
}

#[cfg(test)]
mod test_yaml {
    use super::Yaml;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "


version:                                                                             2
updates:
  - package-ecosystem:                    \"cargo\"
    directory:  \"/\"
    schedule:
      interval:     \"monthly\"
    assignees:
      -     \"hougesen\"
    open-pull-requests-limit:       25

  - package-ecosystem:                              \"github-actions\"
    directory:          \"/\"
    schedule:
        interval:          \"monthly\"
    assignees:
        - \"hougesen\"
    open-pull-requests-limit: 25


        ";

    const EXTENSION: &str = crate::languages::Language::Yaml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Yaml>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Yaml> {
            enabled: false,
            formatter: MdsfFormatter::Single(Yaml::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<Yaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Yaml::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "version: 2
updates:
  - package-ecosystem: \"cargo\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25

  - package-ecosystem: \"github-actions\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(yamlfmt)]
    #[test]
    fn test_yamlfmt() {
        let l = Lang::<Yaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Yaml::YamlFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "version: 2
updates:
  - package-ecosystem: \"cargo\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
  - package-ecosystem: \"github-actions\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(yamlfix)]
    #[test]
    fn test_yamlfix() {
        let l = Lang::<Yaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Yaml::YamlFix),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "---
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
";

        assert_eq!(output, expected_output);
    }
}
