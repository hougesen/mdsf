use mdsf::{caching::CACHE_DIR, get_project_dir};

#[inline]
pub fn run() -> Result<(), std::io::Error> {
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
    fn it_should_remove_cache_directory() -> Result<(), std::io::Error> {
        let cache_dir = get_project_dir().join(CACHE_DIR);

        std::fs::create_dir_all(&cache_dir)?;

        let exists_before = cache_dir.try_exists()?;

        assert!(exists_before);

        super::run()?;

        let exists_after = cache_dir.try_exists()?;

        assert!(!exists_after);

        Ok(())
    }
}
