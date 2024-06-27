use std::{
    env::current_dir,
    sync::{atomic::AtomicU32, Arc},
};

use clap::builder::OsStr;
use mdsf::{
    caching::get_config_hash, cli::FormatCommandArguments, config::MdsfConfig, error::MdsfError,
    handle_file,
};

const MDSF_IGNORE_FILE_NAME: &str = ".mdsfignore";

#[inline]
pub async fn run(args: FormatCommandArguments, dry_run: bool) -> Result<(), MdsfError> {
    mdsf::DEBUG.swap(args.debug, core::sync::atomic::Ordering::Relaxed);

    let conf = if let Some(config_path) = args.config {
        MdsfConfig::load(&config_path)
    } else {
        let path = current_dir()?.join("mdsf.json");

        Ok(MdsfConfig::load(path).unwrap_or_default())
    }?;

    let config_cache_key = if args.cache {
        Some(get_config_hash(&conf))
    } else {
        None
    };

    mdsf::runners::set_javascript_runtime(conf.javascript_runtime);

    let changed_file_count = Arc::new(AtomicU32::new(0));

    let mut tasks = tokio::task::JoinSet::new();

    let shared_config = Arc::new(conf);

    if args.path.is_file() {
        let config_ref = shared_config.clone();

        let changed_file_count_ref = changed_file_count.clone();

        tasks.spawn(async move {
            let was_formatted =
                handle_file(&config_ref, &args.path, dry_run, config_cache_key).await;

            if was_formatted {
                changed_file_count_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });
    } else if args.path.is_dir() {
        let mut walk_builder = ignore::WalkBuilder::new(args.path);

        walk_builder
            .standard_filters(true)
            .parents(true)
            .hidden(true)
            .add_custom_ignore_filename(MDSF_IGNORE_FILE_NAME);

        let md_ext = OsStr::from("md");

        for entry in walk_builder.build().flatten() {
            let file_path = entry.path().to_path_buf();

            if file_path.extension() == Some(&md_ext) {
                let config_ref = shared_config.clone();

                let changed_file_count_ref = changed_file_count.clone();

                let config_cache_key_ref = config_cache_key.clone();

                tasks.spawn(async move {
                    let was_formatted =
                        handle_file(&config_ref, &file_path, dry_run, config_cache_key_ref).await;

                    if was_formatted {
                        changed_file_count_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                });
            }
        }
    } else {
        return Err(MdsfError::FileNotFound(args.path));
    }

    while let Some(_) = tasks.join_next().await {}

    let total_changed_files = changed_file_count.load(std::sync::atomic::Ordering::SeqCst);

    if dry_run && total_changed_files > 0 {
        Err(MdsfError::CheckModeChanges(total_changed_files))
    } else {
        Ok(())
    }
}
