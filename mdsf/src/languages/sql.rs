use crate::{config::MdsfTool, execution::MdsfToolWrapper, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfToolWrapper<MdsfTool>) {
    (
        "sql".to_string(),
        MdsfToolWrapper::Multiple(vec![MdsfToolWrapper::Multiple(vec![
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::SqlFormatter)),
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::SqlfluffFormat)),
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Sqlfmt)),
        ])]),
    )
}
