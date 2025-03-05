use mdsf::{caching::CACHE_DIR, get_project_dir};

#[inline]
pub fn run() {
    let cache_dir = get_project_dir().join(CACHE_DIR);

    if cache_dir.try_exists().is_ok_and(|exists| exists) {
        let _ = std::fs::remove_dir_all(&cache_dir);

        let _ = std::fs::create_dir_all(cache_dir);
    }
}
