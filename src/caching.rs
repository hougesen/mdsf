use std::hash::{DefaultHasher, Hash, Hasher};

use sha2::{Digest, Sha256};

use crate::config::MdsfConfig;

#[inline]
pub fn get_config_hash(config: &MdsfConfig) -> String {
    serde_json::to_string(config).map_or_else(
        |_error| {
            let mut hasher = DefaultHasher::new();

            config.hash(&mut hasher);

            format!("{}", hasher.finish())
        },
        |config_str| {
            let mut h = Sha256::new();

            h.update(config_str);

            format!("{:X}", h.finalize())
        },
    )
}

#[inline]
pub fn hash_text_block(text: &str) -> String {
    let mut h = Sha256::new();

    h.update(text);

    format!("{:X}", h.finalize())
}
