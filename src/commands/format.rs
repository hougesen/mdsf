use std::env::current_dir;

use clap::builder::OsStr;
use mdsf::{cli::FormatCommandArguments, config::MdsfConfig, error::MdsfError, handle_file};

const MDSF_IGNORE_FILE_NAME: &str = ".mdsfignore";

pub fn run(args: FormatCommandArguments, dry_run: bool) -> Result<(), MdsfError> {
    mdsf::DEBUG.swap(args.debug, core::sync::atomic::Ordering::Relaxed);

    let conf = if let Some(config_path) = args.config {
        MdsfConfig::load(&config_path)
    } else {
        let path = current_dir()?.join("mdsf.json");

        Ok(MdsfConfig::load(path).unwrap_or_default())
    }?;

    mdsf::runners::set_javascript_runtime(conf.javascript_runtime);

    let mut changed_file_count = 0;

    if args.path.is_file() {
        changed_file_count += u32::from(handle_file(&conf, &args.path, dry_run)?);
    } else if args.path.is_dir() {
        let mut walk_builder = ignore::WalkBuilder::new(args.path);

        walk_builder
            .standard_filters(true)
            .parents(true)
            .hidden(true)
            .add_custom_ignore_filename(MDSF_IGNORE_FILE_NAME);

        let md_ext = OsStr::from("md");

        for entry in walk_builder.build().flatten() {
            let file_path = entry.path();

            if file_path.extension() == Some(&md_ext) {
                changed_file_count += u32::from(handle_file(&conf, file_path, dry_run)?);
            }
        }
    } else {
        return Err(MdsfError::FileNotFound(args.path));
    }

    if dry_run && changed_file_count > 0 {
        Err(MdsfError::CheckModeChanges(changed_file_count))
    } else {
        Ok(())
    }
}
