use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    terraform_fmt::format_using_terraform_fmt, tofu_fmt::format_using_tofu_fmt, MdsfFormatter,
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Hcl {
    #[default]
    #[serde(rename = "terraform_fmt")]
    TerraformFmt,
    #[serde(rename = "tofu_fmt")]
    TofuFmt,
}

impl Default for Lang<Hcl> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Hcl>::default(),
        }
    }
}

impl Default for MdsfFormatter<Hcl> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Hcl::TerraformFmt),
            Self::Single(Hcl::TofuFmt),
        ])])
    }
}

impl LanguageFormatter for Hcl {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::TerraformFmt => format_using_terraform_fmt(snippet_path),
            Self::TofuFmt => format_using_tofu_fmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_hcl {
    use super::Hcl;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    const INPUT: &str = "resource \"aws_instance\" \"example\" {                         
                                         ami   = \"abc123\"

           network_interface  {
             }
}
";

    const EXTENSION: &str = crate::languages::Language::Hcl.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Hcl>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Hcl> {
            enabled: false,
            formatter: MdsfFormatter::Single(Hcl::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(terraform)]
    #[test]
    fn test_terraform_fmt() {
        let l = Lang::<Hcl> {
            enabled: true,
            formatter: MdsfFormatter::Single(Hcl::TerraformFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "resource \"aws_instance\" \"example\" {
  ami = \"abc123\"

  network_interface {
  }
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(tofu)]
    #[test]
    fn test_tofu_fmt() {
        let l = Lang::<Hcl> {
            enabled: true,
            formatter: MdsfFormatter::Single(Hcl::TofuFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "resource \"aws_instance\" \"example\" {
  ami = \"abc123\"

  network_interface {
  }
}
";

        assert_eq!(output, expected_output);
    }
}
