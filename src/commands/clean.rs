//! Removes compiled binaries and build artifacts from the current directory.

use crate::utils::{filesystem, output};
use anyhow::Result;
use std::fs;

/// Scans the current working directory for `.exe` and `.out` files and deletes
/// them.
///
/// Only top-level files are removed; subdirectories are not traversed.
pub fn execute() -> Result<()> {
    output::info("Cleaning up temporary files and binaries...");

    let current_dir = std::env::current_dir()?;

    let removed = fs::read_dir(current_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext == "exe" || ext == "out")
        })
        .filter(|path| filesystem::remove_file(path).is_ok())
        .count();

    output::success(&format!("Cleaned up {removed} files."));
    Ok(())
}
