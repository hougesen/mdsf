use crate::error::CodegenError;

pub fn setup() -> Result<(), CodegenError> {
    println!("copy latest stable schema");

    let latest_version = env!("CARGO_PKG_VERSION").split('-').next().unwrap();

    let schema_dir = std::path::Path::new("schemas");

    let stable_dir_name = "stable";
    let stable_dir_path = schema_dir.join(stable_dir_name);

    let _ = std::fs::create_dir_all(&stable_dir_path);

    let schema_file_name = "mdsf.schema.json";
    let stable_file_path = schema_dir.join(stable_dir_name).join(schema_file_name);

    std::fs::write(&stable_file_path, "")?;

    let latest_released_version = schema_dir
        .join(format!("v{latest_version}"))
        .join(schema_file_name);

    std::fs::copy(latest_released_version, stable_file_path)?;

    Ok(())
}
