use clap::Parser;
use config::{Cli, Commands, FormatCommandArguments};
use formatters::format_snippet;
use markdown::{generate_markdown, Block};

mod config;
mod formatters;

fn format_file(path: &std::path::Path) -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.md")?;

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

    output.push(Block::Raw(String::new()));

    std::fs::write(path, generate_markdown(output))
}

fn format_command(args: FormatCommandArguments) -> std::io::Result<()> {
    if args.path.is_file() {
        return format_file(&args.path);
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
