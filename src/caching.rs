use std::hash::{DefaultHasher, Hash, Hasher};

use crate::config::MdsfConfig;

pub fn get_config_hash(config: &MdsfConfig) -> String {
    let mut hasher = DefaultHasher::new();

    config.hash(&mut hasher);

    format!("{}", hasher.finish())
}

pub fn hash_text_block(text: &str) -> String {
    let mut hasher = DefaultHasher::new();

    text.hash(&mut hasher);

    format!("{}", hasher.finish())
}
