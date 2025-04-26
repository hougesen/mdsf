use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "csharp".to_string(),
        MdsfFormatter::Single(Tooling::Csharpier),
    )
}
