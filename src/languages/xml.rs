use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    xmlformat::format_using_xmlformat, xmllint::format_using_xmllint, MdsfFormatter,
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Xml {
    #[default]
    #[serde(rename = "xmllint")]
    XmlLint,
    #[serde(rename = "xmlformat")]
    XmlFormat,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Xml::XmlLint),
            Self::Single(Xml::XmlFormat),
        ])])
    }
}

impl LanguageFormatter for Xml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::XmlLint => format_using_xmllint(snippet_path),
            Self::XmlFormat => format_using_xmlformat(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_xml {
    use super::Xml;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

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
            formatter: MdsfFormatter::Single(Xml::XmlLint),
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
            formatter: MdsfFormatter::Single(Xml::XmlLint),
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

    #[test_with::executable(xmlformat)]
    #[test]
    fn test_xmlformat() {
        let l = Lang::<Xml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Xml::XmlFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>";

        assert_eq!(output, expected_output);
    }
}
