use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum YamlFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Yaml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: YamlFormatter,
}

impl Default for Yaml {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: YamlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Yaml {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            YamlFormatter::Prettier => format_using_prettier(snippet_path, true),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_yaml {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Yaml, YamlFormatter};

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
        assert!(Yaml::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Yaml {
            enabled: false,
            formatter: YamlFormatter::Prettier,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Yaml {
            enabled: true,
            formatter: YamlFormatter::Prettier,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
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
}
