use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    alejandra::format_using_alejandra, nixfmt::format_using_nixfmt,
    nixpkgs_fmt::format_using_nixpkgs_fmt, MdsfFormatter,
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Nix {
    #[default]
    #[serde(rename = "nixfmt")]
    Nixfmt,
    #[serde(rename = "nixpkgs-fmt")]
    NixpkgsFmt,
    #[serde(rename = "alejandra")]
    Alejandra,
}

impl Default for Lang<Nix> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Nix>::default(),
        }
    }
}

impl Default for MdsfFormatter<Nix> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Nix::Nixfmt),
            Self::Single(Nix::Alejandra),
            Self::Single(Nix::NixpkgsFmt),
        ])])
    }
}

impl LanguageFormatter for Nix {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Alejandra => format_using_alejandra(snippet_path),
            Self::Nixfmt => format_using_nixfmt(snippet_path),
            Self::NixpkgsFmt => format_using_nixpkgs_fmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Nix {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alejandra => write!(f, "alejandra"),
            Self::Nixfmt => write!(f, "nixfmt"),
            Self::NixpkgsFmt => write!(f, "nixpkgs-fmt"),
        }
    }
}

#[cfg(test)]
mod test_nix {
    use super::Nix;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = r#"{
            lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

    const EXTENSION: &str = crate::languages::Language::Nix.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Nix>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Nix> {
            enabled: false,
            formatter: MdsfFormatter::Single(Nix::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(nixfmt)]
    #[test]
    fn test_nixfmt() {
        let expected_output = r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let l = Lang::<Nix> {
            enabled: true,
            formatter: MdsfFormatter::Single(Nix::Nixfmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(nixpkgs-fmt)]
    #[test]
    fn test_nixpkgs_fmt() {
        let l = Lang::<Nix> {
            enabled: true,
            formatter: MdsfFormatter::Single(Nix::NixpkgsFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"{ lib
, buildPythonPackage
, fetchFromGitHub
, redis
}:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;
        assert_eq!(output, expected_output);
    }

    #[test_with::executable(alejandra)]
    #[test]
    fn test_alejandra() {
        let expected_output = r#"{
  lib,
  buildPythonPackage,
  fetchFromGitHub,
  redis,
}:
buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [redis];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [maintainers.globin];
  };
}
"#;

        let l = Lang::<Nix> {
            enabled: true,
            formatter: MdsfFormatter::Single(Nix::Alejandra),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
