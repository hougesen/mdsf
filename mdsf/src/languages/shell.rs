use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "shell".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Shfmt)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Beautysh)),
        ])]),
    )
}
