use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{rescript_format::format_using_rescript_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum ReScript {
    #[default]
    #[serde(rename = "rescript_format")]
    ReScriptFormat,
}

impl Default for Lang<ReScript> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ReScript>::default(),
        }
    }
}

impl Default for MdsfFormatter<ReScript> {
    #[inline]
    fn default() -> Self {
        Self::Single(ReScript::ReScriptFormat)
    }
}

impl LanguageFormatter for ReScript {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::ReScriptFormat => format_using_rescript_format(snippet_path),
        }
    }
}

impl core::fmt::Display for ReScript {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ReScriptFormat => write!(f, "rescript_format"),
        }
    }
}

#[cfg(test)]
mod test_rescript {
    use super::ReScript;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = r#"module Button = {
  @react.component
  let make = (~count) =>   {
    let times = switch    count {
            | 1          =>   "once"
    | 2  =>         "twice"
    |   n =>      n->Int.toString ++ " times"
     }
     let text =                           `Click me ${times}`

    <button> {text->React.string} </button>
  }
}"#;

    const EXTENSION: &str = crate::languages::Language::ReScript.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<ReScript>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<ReScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(ReScript::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_rescript_format() {
        let l = Lang::<ReScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(ReScript::ReScriptFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"module Button = {
  @react.component
  let make = (~count) => {
    let times = switch count {
    | 1 => "once"
    | 2 => "twice"
    | n => n->Int.toString ++ " times"
    }
    let text = `Click me ${times}`

    <button> {text->React.string} </button>
  }
}
"#;

        assert_eq!(output, expected_output);
    }
}
