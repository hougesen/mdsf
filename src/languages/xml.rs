use schemars::JsonSchema;

use crate::formatters::{xmllint::format_using_xmllint, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Xml {
    #[default]
    #[serde(rename = "xmllint")]
    Xmllint,
}

impl Default for Lang<Xml> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Xml>::default(),
        }
    }
}

impl Default for MdsfFormatter<Xml> {
    #[inline]
    fn default() -> Self {
        Self::Single(Xml::Xmllint)
    }
}

impl LanguageFormatter for Xml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Xmllint => format_using_xmllint(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_xml {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Xml;

    const INPUT: &str = "
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>";

    const EXTENSION: &str = crate::languages::Language::Xml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Xml>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Xml> {
            enabled: false,
            formatter: MdsfFormatter::Single(Xml::Xmllint),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test_with::executable(xmllint)]
    #[test]
    fn test_xmllint() {
        let l = Lang::<Xml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Xml::Xmllint),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

        assert_eq!(output, expected_output);
    }
}
