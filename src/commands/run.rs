//! Runs a compiled binary with optional arguments.

use crate::utils::{output, process};
use anyhow::{bail, Result};

/// Executes `binary` once, forwarding its stdout/stderr to the terminal.
///
/// Uses a 10-second default timeout to guard against infinite loops in
/// competitive-programming solutions.
pub fn execute(binary: &str, args: &[String]) -> Result<()> {
    output::info(&format!("Running {binary}..."));

    let args_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = process::execute_with_timeout(binary, &args_refs, None, 10.0)?;

    if result.success {
        print!("{}", result.stdout);
        Ok(())
    } else if let Some(code) = result.exit_code {
        print!("{}", result.stdout);
        eprintln!("{}", result.stderr);
        bail!("Process exited with error code: {code}");
    } else {
        bail!("Process timed out or was terminated.");
    }
}
