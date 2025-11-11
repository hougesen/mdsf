use crate::{config::MdsfTool, execution::MdsfToolWrapper, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfToolWrapper<MdsfTool>) {
    (
        "ruby".to_string(),
        MdsfToolWrapper::Multiple(vec![MdsfToolWrapper::Multiple(vec![
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Rubocop)),
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Rufo)),
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Rubyfmt)),
            MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Standardrb)),
        ])]),
    )
}
