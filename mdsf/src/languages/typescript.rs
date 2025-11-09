use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "typescript".to_string(),
        MdsfFormatter::Single(MdsfTool::Preset(Tooling::Prettier)),
    )
}
