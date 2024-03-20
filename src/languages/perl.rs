use schemars::JsonSchema;

use crate::formatters::{perltidy::format_using_perltidy, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Perl {
    #[default]
    #[serde(rename = "perltidy")]
    PerlTidy,
}

impl Default for Lang<Perl> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Perl>::default(),
        }
    }
}

impl Default for MdsfFormatter<Perl> {
    #[inline]
    fn default() -> Self {
        Self::Single(Perl::PerlTidy)
    }
}

impl LanguageFormatter for Perl {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::PerlTidy => format_using_perltidy(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_perl {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Perl;

    const INPUT: &str = r#"$_= <<'EOL';
   $url = new URI::URL "http://www/";   die if $url eq "xXx";
EOL
LOOP:{print(" digits"),redo LOOP if/\G\d+\b[,.;]?\s*/gc;print(" lowercase"),
redo LOOP if/\G[a-z]+\b[,.;]?\s*/gc;print(" UPPERCASE"),redo LOOP
if/\G[A-Z]+\b[,.;]?\s*/gc;print(" Capitalized"),
redo LOOP if/\G[A-Z][a-z]+\b[,.;]?\s*/gc;
print(" MiXeD"),redo LOOP if/\G[A-Za-z]+\b[,.;]?\s*/gc;print(
" alphanumeric"),redo LOOP if/\G[A-Za-z0-9]+\b[,.;]?\s*/gc;print(" line-noise"
),redo LOOP if/\G[^A-Za-z0-9]+/gc;print". That's all!\n";}"#;

    const EXTENSION: &str = crate::languages::Language::Perl.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Perl>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Perl> {
            enabled: false,
            formatter: MdsfFormatter::Single(Perl::PerlTidy),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test_with::executable(perltidy)]
    #[test]
    fn test_perltidy() {
        let l = Lang::<Perl> {
            enabled: true,
            formatter: MdsfFormatter::Single(Perl::PerlTidy),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"$_ = <<'EOL';
   $url = new URI::URL "http://www/";   die if $url eq "xXx";
EOL
LOOP: {
    print(" digits"),    redo LOOP if /\G\d+\b[,.;]?\s*/gc;
    print(" lowercase"), redo LOOP if /\G[a-z]+\b[,.;]?\s*/gc;
    print(" UPPERCASE"), redo LOOP
      if /\G[A-Z]+\b[,.;]?\s*/gc;
    print(" Capitalized"),  redo LOOP if /\G[A-Z][a-z]+\b[,.;]?\s*/gc;
    print(" MiXeD"),        redo LOOP if /\G[A-Za-z]+\b[,.;]?\s*/gc;
    print(" alphanumeric"), redo LOOP if /\G[A-Za-z0-9]+\b[,.;]?\s*/gc;
    print(" line-noise"),   redo LOOP if /\G[^A-Za-z0-9]+/gc;
    print ". That's all!\n";
}
"#;

        assert_eq!(output, expected_output);
    }
}
