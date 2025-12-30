use crate::markdown::{table_of_contents, update_markdown_section};

pub fn generate() -> anyhow::Result<()> {
    let path = std::path::Path::new("./CONTRIBUTING.md");

    let mut contents = std::fs::read_to_string(path)?;

    {
        let t = table_of_contents::generate(path)?;

        contents = update_markdown_section(&contents, "toc", &t);

        std::fs::write(path, contents)?;
    };

    Ok(())
}
