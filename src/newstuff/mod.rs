use crate::{
    error::MdsfError,
    formatters::{
        biome::format_using_biome, clang_format::format_using_clang_format,
        crlfmt::format_using_crlfmt, deno_fmt::format_using_deno_fmt, gci::format_using_gci,
        gofmt::format_using_gofmt, gofumpt::format_using_gofumpt,
        goimports::format_using_goimports, goimports_reviser::format_using_goimports_reviser,
        golines::format_using_golines, prettier::format_using_prettier,
        standardjs::format_using_standardjs, MdsfFormatter,
    },
    languages::LanguageFormatter,
    terminal::{
        print_binary_not_in_path, print_error_formatting, print_formatter_info,
        print_formatter_time,
    },
    LineInfo,
};

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Tool {
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "deno_fmt")]
    DenoFmt,
    #[serde(rename = "clang-format")]
    ClangFormat,
    #[serde(rename = "standardjs")]
    Standardjs,

    #[serde(rename = "gofmt")]
    GoFmt,
    #[serde(rename = "gofumpt")]
    GoFumpt,
    #[serde(rename = "goimports")]
    GoImports,
    #[serde(rename = "goimports-reviser")]
    GoImportsReviser,
    #[serde(rename = "crlfmt")]
    CrlFmt,
    #[serde(rename = "gci")]
    GCI,
    #[serde(rename = "golines")]
    GoLines,
}

impl LanguageFormatter for Tool {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Biome => format_using_biome(snippet_path),
            Self::Prettier => format_using_prettier(snippet_path),
            Self::ClangFormat => format_using_clang_format(snippet_path),
            Self::DenoFmt => format_using_deno_fmt(snippet_path),
            Self::Standardjs => format_using_standardjs(snippet_path),
            Self::GoFmt => format_using_gofmt(snippet_path),
            Self::GoFumpt => format_using_gofumpt(snippet_path),
            Self::GoImports => format_using_goimports(snippet_path),
            Self::GoImportsReviser => format_using_goimports_reviser(snippet_path),
            Self::CrlFmt => format_using_crlfmt(snippet_path),
            Self::GCI => format_using_gci(snippet_path),
            Self::GoLines => format_using_golines(snippet_path),
        }
    }
}

impl core::fmt::Display for Tool {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::Biome => write!(f, "biome"),
            Self::DenoFmt => write!(f, "deno_fmt"),
            Self::ClangFormat => write!(f, "clang-format"),
            Self::Standardjs => write!(f, "standardjs"),
            Self::GoFmt => write!(f, "gofmt"),
            Self::GoFumpt => write!(f, "gofumpt"),
            Self::GoImports => write!(f, "goimports"),
            Self::GoImportsReviser => write!(f, "goimports-reviser"),
            Self::CrlFmt => write!(f, "crlfmt"),
            Self::GCI => write!(f, "gci"),
            Self::GoLines => write!(f, "golines"),
        }
    }
}
impl Default for MdsfFormatter<Tool> {
    fn default() -> Self {
        MdsfFormatter::Multiple(Vec::new())
    }
}

impl MdsfFormatter<Tool> {
    #[inline]
    pub fn format(
        &self,
        snippet_path: &std::path::Path,
        info: &LineInfo,
    ) -> Result<Option<String>, MdsfError> {
        Self::format_multiple(&self, snippet_path, info, false)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    pub fn format_multiple(
        formatter: &MdsfFormatter<Tool>,
        snippet_path: &std::path::Path,
        info: &LineInfo,
        nested: bool,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match formatter {
            MdsfFormatter::Single(f) => {
                let formatter_name = f.to_string();

                print_formatter_info(&formatter_name, info);

                let time = std::time::Instant::now();

                let r = f.format_snippet(snippet_path);

                print_formatter_time(&formatter_name, info, time.elapsed());

                if let Err(e) = &r {
                    if let MdsfError::MissingBinary(binary) = e {
                        print_binary_not_in_path(
                            if &formatter_name == binary {
                                formatter_name
                            } else {
                                format!("{binary} ({formatter_name})")
                            }
                            .as_str(),
                        );

                        return Ok((false, None));
                    } else if matches!(e, MdsfError::FormatterError) {
                        print_error_formatting(&formatter_name, info);
                        return Ok((false, None));
                    }
                }

                r
            }

            MdsfFormatter::Multiple(formatters) => {
                let mut r = Ok((true, None));

                for f in formatters {
                    r = Self::format_multiple(f, snippet_path, info, true);

                    if r.as_ref()
                        .is_ok_and(|(should_continue, _code)| !should_continue)
                        && nested
                    {
                        break;
                    }
                }

                r
            }
        }
    }
}
