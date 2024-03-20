use schemars::JsonSchema;

use crate::formatters::{
    beautysh::format_using_beautysh, shfmt::format_using_shfmt, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Shell {
    #[default]
    #[serde(rename = "shfmt")]
    Shfmt,
    #[serde(rename = "beautysh")]
    Beautysh,
}

impl Default for Lang<Shell> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Shell>::default(),
        }
    }
}

impl Default for MdsfFormatter<Shell> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(Shell::Shfmt),
            Self::Single(Shell::Beautysh),
        ])
    }
}

impl LanguageFormatter for Shell {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Shfmt => format_using_shfmt(snippet_path),
            Self::Beautysh => format_using_beautysh(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_shell {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Shell;

    const INPUT: &str = "

#!/bin/sh

       add      ()   {
    echo \"$1\"                 +          \"$2\"
             }








";

    const EXTENSION: &str = crate::languages::Language::Shell.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Shell>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Shell> {
            enabled: false,
            formatter: MdsfFormatter::Single(Shell::Shfmt),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(shfmt)]
    #[test]
    fn test_shfmt() {
        let expected_output = "#!/bin/sh

add() {
\techo \"$1\" + \"$2\"
}
";

        let l = Lang::<Shell> {
            enabled: true,
            formatter: MdsfFormatter::Single(Shell::Shfmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(beautysh)]
    #[test]
    fn test_beautysh() {
        let input = "#!/bin/shell

       add() {
    echo \"$1\" + \"$2\"
             }
";
        let expected_output = "#!/bin/shell

add() {
    echo \"$1\" + \"$2\"
}
";

        let l = Lang::<Shell> {
            enabled: true,
            formatter: MdsfFormatter::Single(Shell::Beautysh),
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
