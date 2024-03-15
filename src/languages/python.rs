use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        autopep8::format_using_autopep8, black::format_using_black, blue::format_using_blue,
        format_multiple, isort::format_using_isort, ruff::format_using_ruff,
        usort::format_using_usort, yapf::format_using_yapf, MdsfFormatter,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum PythonFormatter {
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

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Python {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<PythonFormatter>,
}

impl Default for Python {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<PythonFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<PythonFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(PythonFormatter::Ruff),
            Self::Multiple(vec![
                Self::Multiple(vec![
                    Self::Single(PythonFormatter::Usort),
                    Self::Single(PythonFormatter::Isort),
                ]),
                Self::Multiple(vec![
                    Self::Single(PythonFormatter::Blue),
                    Self::Single(PythonFormatter::Black),
                    Self::Single(PythonFormatter::Yapf),
                    Self::Single(PythonFormatter::Autopep8),
                ]),
            ]),
        ])
    }
}

impl LanguageFormatter<PythonFormatter> for Python {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &PythonFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        println!("formatter: {formatter:#?}");

        match formatter {
            PythonFormatter::Autopep8 => format_using_autopep8(snippet_path),
            PythonFormatter::Black => format_using_black(snippet_path),
            PythonFormatter::Blue => format_using_blue(snippet_path),
            PythonFormatter::Ruff => format_using_ruff(snippet_path),
            PythonFormatter::Yapf => format_using_yapf(snippet_path),
            PythonFormatter::Isort => format_using_isort(snippet_path),
            PythonFormatter::Usort => format_using_usort(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_python {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Python, PythonFormatter};

    const INPUT: &str = "def add( a: int ,  b:int)->int: return a+b";

    const EXTENSION: &str = crate::languages::Language::Python.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Python::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Python {
            enabled: false,
            formatter: MdsfFormatter::Single(PythonFormatter::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_black() {
        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Black),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Blue),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Ruff),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Autopep8),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Yapf),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Isort),
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

        let l = Python {
            enabled: true,
            formatter: MdsfFormatter::Single(PythonFormatter::Usort),
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
