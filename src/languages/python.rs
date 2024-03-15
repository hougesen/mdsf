use schemars::JsonSchema;

use crate::formatters::{
    autopep8::format_using_autopep8, black::format_using_black, blue::format_using_blue,
    isort::format_using_isort, ruff::format_using_ruff, usort::format_using_usort,
    yapf::format_using_yapf, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Python {
    #[default]
    #[serde(rename = "ruff")]
    Ruff,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "yapf")]
    Yapf,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "autopep8")]
    Autopep8,
    #[serde(rename = "isort")]
    Isort,
    #[serde(rename = "usort")]
    Usort,
}

impl LanguageFormatter for Python {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Autopep8 => format_using_autopep8(snippet_path),
            Self::Black => format_using_black(snippet_path),
            Self::Blue => format_using_blue(snippet_path),
            Self::Ruff => format_using_ruff(snippet_path),
            Self::Yapf => format_using_yapf(snippet_path),
            Self::Isort => format_using_isort(snippet_path),
            Self::Usort => format_using_usort(snippet_path),
        }
    }
}

impl Default for MdsfFormatter<Python> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Python::Ruff),
            Self::Multiple(vec![
                Self::Multiple(vec![
                    Self::Single(Python::Usort),
                    Self::Single(Python::Isort),
                ]),
                Self::Multiple(vec![
                    Self::Single(Python::Blue),
                    Self::Single(Python::Black),
                    Self::Single(Python::Yapf),
                    Self::Single(Python::Autopep8),
                ]),
            ]),
        ])])
    }
}

impl Default for Lang<Python> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Python>::default(),
        }
    }
}

#[cfg(test)]
mod test_python {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Python;

    const INPUT: &str = "def add( a: int ,  b:int)->int: return a+b";

    const EXTENSION: &str = crate::languages::Language::Python.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Python>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Python> {
            enabled: false,
            formatter: MdsfFormatter::Single(Python::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_black() {
        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Black),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_blue() {
        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Blue),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_ruff() {
        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Ruff),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_autopep8() {
        let expected_output = "def add(a: int,  b: int) -> int: return a+b\n";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Autopep8),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_yapf() {
        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Yapf),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_isort() {
        let input = "from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
";

        let expected_output = "import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Isort),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_usort() {
        let input = "from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
";

        let expected_output = "import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
";

        let l = Lang::<Python> {
            enabled: true,
            formatter: MdsfFormatter::Single(Python::Usort),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
