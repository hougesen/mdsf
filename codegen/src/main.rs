use anyhow::{Ok, Result};

mod cargo;
mod language_to_filetype;
mod readme_tooling;
mod schema;

fn main() -> Result<()> {
    schema::generate()?;

    language_to_filetype::generate()?;

    readme_tooling::generate()?;

    Ok(())
}
