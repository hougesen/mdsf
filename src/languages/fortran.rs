use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{findent::format_using_findent, fprettify::format_using_fprettify, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Fortran {
    #[default]
    #[serde(rename = "fprettify")]
    Fprettify,
    #[serde(rename = "findent")]
    Findent,
}

impl Default for Lang<Fortran> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Fortran>::default(),
        }
    }
}

impl Default for MdsfFormatter<Fortran> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Fortran::Fprettify),
            Self::Single(Fortran::Findent),
        ])])
    }
}

impl LanguageFormatter for Fortran {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Fprettify => format_using_fprettify(snippet_path),
            Self::Findent => format_using_findent(snippet_path),
        }
    }
}

impl core::fmt::Display for Fortran {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fprettify => write!(f, "fprettify"),
            Self::Findent => write!(f, "findent"),
        }
    }
}

#[cfg(test)]
mod test_fortran {
    use super::Fortran;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "program demo
integer :: endif,if,elseif
integer,DIMENSION(2) :: function
endif=3;if=2
if(endif==2)then
endif=5
elseif=if+4*(endif+&
2**10)
elseif(endif==3)then
function(if)=endif/elseif
print*,endif
endif
end program";

    const EXTENSION: &str = crate::languages::Language::Fortran.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Fortran>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Fortran> {
            enabled: false,
            formatter: MdsfFormatter::Single(Fortran::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(fprettify)]
    #[test]
    fn test_fprettify() {
        let l = Lang::<Fortran> {
            enabled: true,
            formatter: MdsfFormatter::Single(Fortran::Fprettify),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "program demo
   integer :: endif, if, elseif
   integer, DIMENSION(2) :: function
   endif = 3; if = 2
   if (endif == 2) then
      endif = 5
      elseif = if + 4*(endif + &
                       2**10)
   elseif (endif == 3) then
      function(if) = endif/elseif
      print *, endif
   end if
end program
";

        assert_eq!(output, expected_output);
    }
}
