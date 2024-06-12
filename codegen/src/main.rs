use anyhow::{Ok, Result};

mod language_to_filetype;
mod schema;

fn main() -> Result<()> {
    schema::generate()?;

    language_to_filetype::generate_language_to_ft()?;

    Ok(())
}
