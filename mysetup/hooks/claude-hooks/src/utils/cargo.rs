use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn find_cargo_project_root(file_path: &Path) -> Result<PathBuf> {
    let mut current_dir = file_path.parent().unwrap_or(file_path);

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(current_dir.to_path_buf());
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => return Err(anyhow::anyhow!("No Cargo.toml found in parent directories")),
        }
    }
}
