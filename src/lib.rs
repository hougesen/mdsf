use core::sync::atomic::AtomicBool;

use config::MdsfConfig;
use error::MdsfError;
use formatters::format_snippet;
use languages::Language;
use pulldown_cmark::CowStr;
use pulldown_cmark_to_cmark::cmark_resume_with_options;

pub mod cli;
pub mod config;
pub mod error;
pub mod formatters;
pub mod languages;
mod runners;

#[cfg(test)]
pub static DEBUG: AtomicBool = AtomicBool::new(true);
#[cfg(not(test))]
pub static DEBUG: AtomicBool = AtomicBool::new(false);

#[inline]
fn write_unchanged_line(path: &std::path::Path, dur: core::time::Duration) {
    #[cfg(target_os = "windows")]
    let pre = "";
    #[cfg(not(target_os = "windows"))]
    let pre = "\u{1b}[2m";

    #[cfg(target_os = "windows")]
    let post = "";
    #[cfg(not(target_os = "windows"))]
    let post = "\u{1b}[0m";

    println!(
        "{pre}{} {}ms (unchanged){post}",
        path.display(),
        dur.as_millis()
    );
}

#[inline]
fn write_changed_line(path: &std::path::Path, dur: core::time::Duration) {
    println!("{} {}ms", path.display(), dur.as_millis());
}

#[inline]
pub fn format_file(config: &MdsfConfig, path: &std::path::Path) -> Result<(), MdsfError> {
    let time = std::time::Instant::now();

    let input = std::fs::read_to_string(path)?;

    if input.is_empty() {
        let duration = time.elapsed();
        write_unchanged_line(path, duration);
        return Ok(());
    }

    let parser = pulldown_cmark::Parser::new_ext(&input, pulldown_cmark::Options::all());

    let mut output = String::with_capacity(input.len() + 128);

    let mut codeblock_language = None;

    let mut state = None;

    let mut modified = false;

    for event in parser {
        let ev = match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(
                pulldown_cmark::CodeBlockKind::Fenced(l),
            )) => {
                codeblock_language = Language::maybe_from_str(&l);

                pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(
                    pulldown_cmark::CodeBlockKind::Fenced(l),
                ))
            }

            pulldown_cmark::Event::End(t) => {
                if codeblock_language.is_some() {
                    codeblock_language = None;
                }

                pulldown_cmark::Event::End(t)
            }
            pulldown_cmark::Event::Text(text) => {
                if let Some(language) = &codeblock_language {
                    let formatted = CowStr::from(format_snippet(config, language, &text));
                    modified = true;
                    pulldown_cmark::Event::Text(formatted)
                } else {
                    pulldown_cmark::Event::Text(text)
                }
            }
            e => e,
        };

        state = cmark_resume_with_options(
            core::iter::once(ev),
            &mut output,
            state.take(),
            pulldown_cmark_to_cmark::Options {
                code_block_token_count: 3,
                // Default for prettier
                list_token: '-',
                emphasis_token: '_',
                ..Default::default()
            },
        )
        .map_err(MdsfError::from)?
        .into();
    }

    if let Some(s) = state {
        s.finalize(&mut output).map_err(MdsfError::from)?;
    }

    let mut output = output.trim().to_owned();

    if config.markdown.enabled {
        if !output.is_empty() {
            output = format_snippet(config, &Language::Markdown, &output);
            modified = true;
        }
    } else {
        output.push('\n');
    }

    let duration = time.elapsed();

    if modified && output != input {
        std::fs::write(path, output)?;

        write_changed_line(path, duration);

        return Ok(());
    }

    write_unchanged_line(path, duration);

    Ok(())
}
