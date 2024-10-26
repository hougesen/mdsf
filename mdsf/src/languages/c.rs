use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("c".to_string(), MdsfFormatter::Single(Tooling::ClangFormat))
}
