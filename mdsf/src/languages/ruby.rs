use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "ruby".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Rubocop)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Rufo)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Rubyfmt)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Standardrb)),
        ])]),
    )
}
