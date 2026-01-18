use std::fs;
use std::path::{Path, PathBuf};

fn vault_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("rvault"))
}

pub fn vault_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("vault.bin"))
}

pub fn ensure_dir() -> Result<(), String> {
    fs::create_dir_all(vault_dir()?).map_err(|e| e.to_string())
}

pub fn exists(path: &Path) -> bool {
    path.exists()
}

pub fn read_all(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| e.to_string())
}

/// Atomic write: temp file then rename (atomic on POSIX when same filesystem).
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    ensure_dir()?;

    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}
