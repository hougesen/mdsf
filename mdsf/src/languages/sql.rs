use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "sql".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::SqlFormatter)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::SqlfluffFormat)),
            MdsfFormatter::Single(MdsfTool::Preset(Tooling::Sqlfmt)),
        ])]),
    )
}
