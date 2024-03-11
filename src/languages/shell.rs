use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::shfmt::format_using_shfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ShellFormatter {
    #[default]
    #[serde(rename = "shfmt")]
    Shfmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Shell {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ShellFormatter,
}

impl Default for Shell {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ShellFormatter::default(),
        }
    }
}

impl LanguageFormatter for Shell {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ShellFormatter::Shfmt => format_using_shfmt(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_shell {
    use crate::{
        formatters::setup_snippet,
        languages::{shell::ShellFormatter, Language, LanguageFormatter},
    };

    use super::Shell;

    const INPUT: &str = "

#!/bin/sh

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";

    const EXTENSION: &str = Language::Shell.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Shell::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Shell {
            enabled: false,
            formatter: ShellFormatter::Shfmt,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_shfmt() {
        let expected_output = "#!/bin/sh

add() {
\techo \"$1\" + \"$2\"
}
";

        let l = Shell {
            enabled: true,
            formatter: ShellFormatter::Shfmt,
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
