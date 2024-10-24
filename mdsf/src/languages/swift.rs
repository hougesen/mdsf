use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "swift".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::SwiftFormat),
            MdsfFormatter::Single(Tooling::Swiftformat),
        ])]),
    )
}
