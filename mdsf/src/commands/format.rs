use std::{
    env::current_dir,
    sync::{atomic::AtomicU32, Arc},
};

use clap::builder::OsStr;
use mdsf::{
    caching::get_config_hash,
    cli::{read_stdin, FormatCommandArguments},
    config::MdsfConfig,
    error::MdsfError,
    execution::setup_snippet,
    format_file, handle_file,
};
use threadpool::ThreadPool;

const MDSF_IGNORE_FILE_NAME: &str = ".mdsfignore";

#[inline]
fn determine_threads_to_use(argument: Option<usize>) -> usize {
    if let Some(thread_arg) = argument {
        if thread_arg > 0 {
            return thread_arg.to_owned();
        }
    }

    if let Ok(available_threads) = std::thread::available_parallelism().map(usize::from) {
        if available_threads > 0 {
            return available_threads;
        }
    }

    1
}

#[inline]
pub fn run(args: FormatCommandArguments, dry_run: bool) -> Result<(), MdsfError> {
    let mut conf = if let Some(config_path) = args.config {
        MdsfConfig::load(config_path)
    } else {
        let path = current_dir()?.join("mdsf.json");

        Ok(MdsfConfig::load(path).unwrap_or_default())
    }?;

    conf.setup_language_aliases()?;

    let config_cache_key = if args.cache {
        Some(get_config_hash(&conf))
    } else {
        None
    };

    mdsf::runners::set_javascript_runtime(conf.javascript_runtime);

    let changed_file_count = Arc::new(AtomicU32::new(0));

    if args.stdin {
        let stdin_input = read_stdin().map_err(MdsfError::ReadStdinError)?;

        let f = setup_snippet(&stdin_input, "md")?;

        let (was_formatted, output) = format_file(
            &conf,
            f.path(),
            &stdin_input,
            args.timeout.unwrap_or_default(),
            args.debug,
        );

        if was_formatted {
            changed_file_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        if !dry_run {
            println!("{output}");
        }
    } else if let Some(first_path) = args.input.first() {
        let mut walk_builder = ignore::WalkBuilder::new(first_path);

        walk_builder
            .standard_filters(true)
            .parents(true)
            .hidden(true)
            .add_custom_ignore_filename(MDSF_IGNORE_FILE_NAME);

        args.input.iter().skip(1).for_each(|p| {
            walk_builder.add(p);
        });

        let md_ext = OsStr::from("md");

        let thread_count = determine_threads_to_use(args.threads).max(1);

        let pool = ThreadPool::new(thread_count);

        let shared_config = Arc::new(conf);

        for entry in walk_builder.build().flatten() {
            let file_path = entry.path().to_path_buf();

            if file_path.extension() == Some(&md_ext) {
                let config_ref = shared_config.clone();

                let changed_file_count_ref = changed_file_count.clone();

                let config_cache_key_ref = config_cache_key.clone();

                pool.execute(move || {
                    let was_formatted = handle_file(
                        &config_ref,
                        &file_path,
                        dry_run,
                        config_cache_key_ref,
                        args.timeout.unwrap_or_default(),
                        args.debug,
                    );

                    if was_formatted {
                        changed_file_count_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                });
            }
        }

        pool.join();
    } else {
        return Err(MdsfError::MissingInput);
    }

    let total_changed_files = changed_file_count.load(std::sync::atomic::Ordering::SeqCst);

    if dry_run && total_changed_files > 0 {
        Err(MdsfError::CheckModeChanges(total_changed_files))
    } else {
        Ok(())
    }
}
