use mdsf::{
    caching::hash_config,
    cli::{FormatCommandArguments, read_stdin},
    config::MdsfConfig,
    error::MdsfError,
    execution::setup_snippet,
    format_file, handle_file,
    terminal::print_config_schema_version_mismatch,
};
use threadpool::ThreadPool;

const MDSF_IGNORE_FILE_NAME: &str = ".mdsfignore";

#[inline]
fn determine_threads_to_use(argument: Option<usize>) -> usize {
    if let Some(thread_arg) = argument
        && thread_arg > 0
    {
        return thread_arg.to_owned();
    }

    if let Ok(available_threads) = std::thread::available_parallelism().map(core::num::NonZero::get)
        && available_threads > 0
    {
        return available_threads;
    }

    1
}

#[inline]
pub fn run(args: FormatCommandArguments, dry_run: bool) -> Result<(), MdsfError> {
    let mut conf = args
        .config
        .map_or_else(MdsfConfig::auto_load, MdsfConfig::load)?;

    if let Some((version, false)) = conf.parse_schema_version()
        && !(version == "development" || version == "stable")
    {
        print_config_schema_version_mismatch(version);
    }

    conf.setup_language_aliases()?;

    let config_hash = if args.cache {
        Some(hash_config(&conf))
    } else {
        None
    };

    let changed_file_count = std::sync::Arc::new(core::sync::atomic::AtomicU32::new(0));

    let on_missing_tool_binary = args
        .on_missing_tool_binary
        .unwrap_or_else(|| conf.on_missing_tool_binary.unwrap_or_default());

    let on_missing_language_definition = args
        .on_missing_language_definition
        .unwrap_or_else(|| conf.on_missing_language_definition.unwrap_or_default());

    if args.stdin {
        let stdin_input = read_stdin().map_err(MdsfError::ReadStdin)?;

        let f = setup_snippet(&stdin_input, ".md")?;

        let (was_formatted, output) = format_file(
            &conf,
            f.path(),
            &stdin_input,
            args.timeout.unwrap_or_default(),
            args.debug,
            on_missing_tool_binary,
            on_missing_language_definition,
        );

        if was_formatted {
            changed_file_count.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        }

        if !dry_run {
            println!("{output}");
        }
    } else if let Some(first_path) = args.input.first() {
        if conf.files.extensions.is_empty() {
            return Err(MdsfError::EmptyFilesExtensions);
        }

        let mut walk_builder = ignore::WalkBuilder::new(first_path);

        walk_builder
            .standard_filters(true)
            .parents(true)
            .hidden(true)
            .add_custom_ignore_filename(MDSF_IGNORE_FILE_NAME);

        args.input.iter().skip(1).for_each(|p| {
            walk_builder.add(p);
        });

        let thread_count = determine_threads_to_use(args.threads).max(1);

        let pool = ThreadPool::new(thread_count);

        let shared_config = std::sync::Arc::new(conf);

        for entry in walk_builder.build().flatten() {
            let file_path = entry.path().to_path_buf();

            if file_path.extension().is_some_and(|ext_os| {
                ext_os
                    .to_str()
                    .is_some_and(|ext_str| shared_config.files.is_enabled_file_extension(ext_str))
            }) {
                let config_ref = shared_config.clone();

                let changed_file_count_ref = changed_file_count.clone();

                let config_hash_clone = config_hash.clone();

                pool.execute(move || {
                    let was_formatted = handle_file(
                        &config_ref,
                        &file_path,
                        dry_run,
                        config_hash_clone,
                        args.timeout.unwrap_or_default(),
                        args.debug,
                        on_missing_tool_binary,
                        on_missing_language_definition,
                    );

                    if was_formatted {
                        changed_file_count_ref.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    }
                });
            }
        }

        pool.join();
    } else {
        return Err(MdsfError::MissingInput);
    }

    let total_changed_files = changed_file_count.load(core::sync::atomic::Ordering::SeqCst);

    if dry_run && total_changed_files > 0 {
        Err(MdsfError::CheckModeChanges(total_changed_files))
    } else {
        Ok(())
    }
}
