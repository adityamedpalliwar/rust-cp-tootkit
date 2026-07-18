//! Compiles a C++ source file using `g++` with configurable flags.

use crate::utils::{config, output, process};
use anyhow::{bail, Result};
use std::path::Path;

/// Compiles `file` with `g++`, producing an executable at `output` (or a
/// platform-appropriate default derived from the source filename).
///
/// Compilation uses a generous 60-second timeout to accommodate large
/// translation units.
pub fn execute(
    file: &str,
    output_path: Option<&str>,
    cpp_version: &str,
    optimize: bool,
) -> Result<()> {
    output::info(&format!("Compiling {file}..."));

    let input_path = Path::new(file);
    if !input_path.exists() {
        bail!("Source file '{file}' does not exist.");
    }

    let default_output = input_path.with_extension(std::env::consts::EXE_EXTENSION);
    let default_output_str = default_output.to_string_lossy();
    let out_file = output_path.unwrap_or(&default_output_str);

    let args = config::get_default_compile_args(file, out_file, cpp_version, optimize);
    let args_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = process::execute_with_timeout("g++", &args_refs, None, 60.0)?;

    if result.success {
        output::success(&format!("Successfully compiled to {out_file}"));
        Ok(())
    } else {
        output::error("Compilation failed.");
        eprintln!("{}", result.stderr);
        bail!("Compilation error");
    }
}
