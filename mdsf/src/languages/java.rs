use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "java".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::GoogleJavaFormat),
            MdsfFormatter::Single(Tooling::ClangFormat),
        ])]),
    )
}
