use std::hash::{DefaultHasher, Hash, Hasher};

use sha2::{Digest, Sha256};

use crate::{config::MdsfConfig, get_project_dir};

pub const CACHE_DIR: &str = "caches/";

pub struct CacheEntry {
    config_hash: String,

    file_path_hash: String,

    file_content_hash: String,
}

impl CacheEntry {
    #[inline]
    pub fn new(config_hash: String, file_path: &std::path::Path, file_content: &str) -> Self {
        Self {
            config_hash,
            file_path_hash: hash_text_block(&file_path.to_string_lossy()),
            file_content_hash: hash_text_block(file_content),
        }
    }

    #[inline]
    fn to_path(&self) -> std::path::PathBuf {
        get_project_dir().join(CACHE_DIR).join(format!(
            "{}/{}/{}/{}",
            env!("CARGO_PKG_VERSION"),
            self.config_hash,
            self.file_path_hash,
            self.file_content_hash
        ))
    }

    #[inline]
    pub fn get(&self) -> Option<String> {
        std::fs::read_to_string(self.to_path()).ok()
    }

    pub fn set(&self, content: &str) -> std::io::Result<()> {
        let p = self.to_path();

        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(p, content)
    }
}

#[inline]
pub fn hash_config(config: &MdsfConfig) -> String {
    serde_json::to_string(config).map_or_else(
        |_error| {
            let mut hasher = DefaultHasher::new();

            config.hash(&mut hasher);

            format!("{}", hasher.finish())
        },
        |config_str| hash_text_block(&config_str),
    )
}

#[cfg(test)]
mod test_hash_config {
    use crate::{caching::hash_config, config::MdsfConfig};

    #[test]
    fn it_should_be_deterministic() {
        assert_eq!(
            hash_config(&MdsfConfig::default()),
            hash_config(&MdsfConfig::default()),
        );
    }
}

#[inline]
pub fn hash_text_block(text: &str) -> String {
    let mut h = Sha256::new();

    h.update(text);

    format!("{:X}", h.finalize())
}

#[cfg(test)]
mod test_hash_text_block {
    use crate::caching::hash_text_block;

    #[test]
    fn it_should_be_deterministic() {
        assert_eq!(
            hash_text_block("mads was here"),
            hash_text_block("mads was here"),
        );
    }
}
