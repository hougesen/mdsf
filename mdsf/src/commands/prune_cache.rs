use mdsf::{caching::CACHE_DIR, get_project_dir};

#[inline]
pub fn run() -> std::io::Result<()> {
    let cache_dir = get_project_dir().join(CACHE_DIR);

    let exists = cache_dir.try_exists()?;

    if exists {
        std::fs::remove_dir_all(&cache_dir)?;
    }

    Ok(())
}

#[cfg(test)]
mod test_run {
    use mdsf::{caching::CACHE_DIR, get_project_dir};

    #[test]
    fn it_should_remove_cache_directory() {
        let cache_dir = get_project_dir().join(CACHE_DIR);

        std::fs::create_dir_all(&cache_dir).expect("it to create dirs if missing");

        let exists_before = cache_dir
            .try_exists()
            .expect("it to determine if it exists");

        assert!(exists_before);

        super::run().expect("it to remove the directory");

        let exists_after = cache_dir
            .try_exists()
            .expect("it to determine if it exists");

        assert!(!exists_after);
    }
}
