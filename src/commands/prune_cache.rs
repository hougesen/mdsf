use std::env::current_dir;

use mdsf::{caching::get_config_hash, cli::CachePruneArguments, config::MdsfConfig};

#[inline]
fn get_config() -> MdsfConfig {
    if let Ok(p) = current_dir().map(|d| d.join("mdsf.json")) {
        MdsfConfig::load(p).unwrap_or_default()
    } else {
        MdsfConfig::default()
    }
}

#[inline]
pub fn run(args: CachePruneArguments) {
    let cache_dir = std::path::PathBuf::from(".mdsf-cache/caches");

    if let Ok(true) = cache_dir.try_exists() {
        if args.all {
            let _ = std::fs::remove_dir_all(&cache_dir);
            let _ = std::fs::create_dir_all(cache_dir);
        } else {
            let config_key = get_config_hash(&get_config());

            if let Ok(entries) = cache_dir.read_dir() {
                for entry in entries.flatten() {
                    if entry.file_name().as_os_str() != config_key.as_str()
                        && entry.metadata().is_ok_and(|path| path.is_dir())
                    {
                        let _ = std::fs::remove_dir_all(entry.path());
                    }
                }
            }
        }
    }
}
