use languages::format_snippet;
use markdown::{generate_markdown, Block};

mod languages;

fn main() -> std::io::Result<()> {
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

    std::fs::write("output.md", generate_markdown(output))?;

    Ok(())
}
