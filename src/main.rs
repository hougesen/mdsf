use clap::{builder::OsStr, Parser};
use cli::{Cli, Commands, FormatCommandArguments};
use formatters::format_snippet;
use languages::Language;
use pulldown_cmark::CowStr;
use pulldown_cmark_to_cmark::cmark_resume_with_options;

mod cli;
mod formatters;
mod languages;

fn format_file(path: &std::path::Path) -> std::io::Result<()> {
    println!("Formatting {path:#?}");

    let input = std::fs::read_to_string(path)?;

    let parser = pulldown_cmark::Parser::new_ext(&input, pulldown_cmark::Options::all());

    let mut output = String::with_capacity(input.len() + 128);

    let mut modified = false;

    let mut codeblock_language = None;

    let mut state = None;

    for event in parser {
        let ev = match event {
            pulldown_cmark::Event::Start(start) => {
                if let pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(l)) =
                    &start
                {
                    codeblock_language = Language::from_str(l);
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
                    let formatted = CowStr::from(format_snippet(language, &text));

                    if formatted != text {
                        modified = true;
                    }

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
        .unwrap()
        .into();
    }

    if modified {
        if let Some(s) = state {
            s.finalize(&mut output).unwrap();
        }
        println!("{path:#?} was formatted");
        return std::fs::write(path, output);
    }

    println!("{path:#?} was not changed");
    Ok(())
}

fn format_command(args: FormatCommandArguments) -> std::io::Result<()> {
    if args.path.is_file() {
        format_file(&args.path)?;
    } else {
        for entry in ignore::WalkBuilder::new(args.path)
            .git_ignore(true)
            .require_git(false)
            .hidden(true)
            .build()
            .flatten()
        {
            let file_path = entry.path();

            if file_path.extension() == Some(&OsStr::from("md")) {
                format_file(file_path)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let command_result = match Cli::parse().command {
        Commands::Format(args) => format_command(args),
    };

    if let Err(error) = command_result {
        eprintln!("{error}");
    }
}
