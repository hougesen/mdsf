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

#[inline]
pub fn format_file(config: &MdsfConfig, path: &std::path::Path) -> Result<(), MdsfError> {
    println!("Formatting {path:#?}");

    let input = std::fs::read_to_string(path)?;

    if input.is_empty() {
        println!("{path:#?} was not changed");
        return Ok(());
    }

    let parser = pulldown_cmark::Parser::new_ext(&input, pulldown_cmark::Options::all());

    let mut output = String::with_capacity(input.len() + 128);

    let mut codeblock_language = None;

    let mut state = None;

    for event in parser {
        let ev = match event {
            pulldown_cmark::Event::Start(start) => {
                if let pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(l)) =
                    &start
                {
                    codeblock_language = Language::maybe_from_str(l);
                }

                pulldown_cmark::Event::Start(start)
            }
            pulldown_cmark::Event::End(end) => {
                if codeblock_language.is_some() {
                    codeblock_language = None;
                }

                pulldown_cmark::Event::End(end)
            }
            pulldown_cmark::Event::Text(text) => {
                if let Some(language) = &codeblock_language {
                    let formatted = CowStr::from(format_snippet(config, language, &text));

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
                ..Default::default()
            },
        )
        .map_err(MdsfError::from)?
        .into();
    }

    if let Some(s) = state {
        s.finalize(&mut output).map_err(MdsfError::from)?;
    }

    if config.markdown.enabled && !output.is_empty() {
        output = format_snippet(config, &Language::Markdown, &output);
    }

    if input != output && (input.trim().is_empty() == output.trim().is_empty()) {
        println!("{path:#?} was formatted");
        return std::fs::write(path, output).map_err(MdsfError::from);
    }

    println!("{path:#?} was not changed");

    Ok(())
}
