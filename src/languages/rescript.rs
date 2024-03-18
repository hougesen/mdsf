use schemars::JsonSchema;

use crate::formatters::{rescript_format::format_using_rescript_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ReScriptFormat => format_using_rescript_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_rescript {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::ReScript;

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
        .format(snippet_path)
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
            .format(snippet_path)
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
