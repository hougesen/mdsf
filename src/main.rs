use clap::{builder::OsStr, Parser};
use cli::{Cli, Commands, FormatCommandArguments};
use formatters::format_snippet;
use markdown::{generate_markdown, Block};

mod cli;
mod formatters;
mod languages;

fn format_file(path: &std::path::Path) -> std::io::Result<()> {
    println!("Formatting {path:#?}");

    let input = std::fs::read_to_string(path)?;

    let tokens = markdown::tokenize(&input);

    let mut output = Vec::new();

    for token in tokens {
        if let Block::CodeBlock(Some(language), c) = token {
            let formatted = format_snippet(&language, c);

            output.push(Block::CodeBlock(Some(language), formatted));
        } else {
            output.push(token);
        }
    }

    output.push(Block::Paragraph(Vec::new()));

    let mut s = generate_markdown(output).trim().to_owned();

    s.push('\n');

    std::fs::write(path, s)
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
