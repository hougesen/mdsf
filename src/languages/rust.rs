use schemars::JsonSchema;

use crate::formatters::{rustfmt::format_using_rustfmt, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Rust {
    #[default]
    #[serde(rename = "rustfmt")]
    RustFmt,
}

impl Default for Lang<Rust> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Rust>::default(),
        }
    }
}

impl Default for MdsfFormatter<Rust> {
    #[inline]
    fn default() -> Self {
        Self::Single(Rust::RustFmt)
    }
}

impl LanguageFormatter for Rust {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::RustFmt => format_using_rustfmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_rust {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Rust;

    const INPUT: &str = "pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    ";

    const EXTENSION: &str = crate::languages::Language::Rust.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Rust>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Rust> {
            enabled: false,
            formatter: MdsfFormatter::Single(Rust::RustFmt),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(rustfmt)]
    #[test]
    fn test_rustfmt() {
        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let l = Lang::<Rust> {
            enabled: true,
            formatter: MdsfFormatter::Single(Rust::RustFmt),
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
