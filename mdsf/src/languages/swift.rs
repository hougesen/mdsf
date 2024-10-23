use crate::{formatters::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "swift".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::AppleSwiftFormat),
            MdsfFormatter::Single(Tooling::NicklockwoodSwiftFormat),
        ])]),
    )
}
