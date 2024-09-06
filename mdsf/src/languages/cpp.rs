use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "cpp".to_string(),
        MdsfFormatter::Single(Tooling::ClangFormat),
    )
}
