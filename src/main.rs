use clap::{builder::OsStr, Parser};
use cli::{Cli, Commands, FormatCommandArguments};
use formatters::format_snippet;
use languages::Language;
use pulldown_cmark::{CowStr, Options};

mod cli;
mod formatters;
mod languages;

fn format_file(path: &std::path::Path) -> std::io::Result<()> {
    println!("Formatting {path:#?}");

    let input = std::fs::read_to_string(path)?;

    let parser = pulldown_cmark::Parser::new_ext(&input, Options::all());

    let mut output = String::with_capacity(input.len() + 128);

    let mut codeblock_language = None;

    let events = parser.map(|event| match event {
        pulldown_cmark::Event::Start(start) => {
            match &start {
                pulldown_cmark::Tag::CodeBlock(block) => {
                    match &block {
                        pulldown_cmark::CodeBlockKind::Fenced(l) => {
                            codeblock_language = Language::from_str(l);
                        }
                        pulldown_cmark::CodeBlockKind::Indented => {}
                    };
                }
                _ => {}
            };

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
                let formatted = format_snippet(language, text.to_string());

                return pulldown_cmark::Event::Text(CowStr::from(formatted));
            };

            pulldown_cmark::Event::Text(text)
        }
        e => e,
    });

    std::fs::write(path, output)
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
