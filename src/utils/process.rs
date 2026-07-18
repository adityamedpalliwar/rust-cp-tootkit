use std::borrow::Cow;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use thiserror::Error;
use wait_timeout::ChildExt;

/// Errors that can occur during child process execution.
#[derive(Error, Debug)]
pub enum ProcessError {
    /// The process could not be spawned or produced an I/O error.
    #[error("Failed to execute process: {0}")]
    ExecutionFailed(String),

    /// The process did not exit within the allowed time budget.
    #[error("Process timed out after {0} seconds")]
    Timeout(f64),
}

/// The captured result of a completed child process.
pub struct ProcessResult {
    /// Whether the process exited with a zero status code.
    pub success: bool,
    /// Captured standard output, decoded as lossy UTF-8.
    pub stdout: String,
    /// Captured standard error, decoded as lossy UTF-8.
    pub stderr: String,
    /// The numeric exit code, if available (may be `None` on signal termination).
    pub exit_code: Option<i32>,
}

/// Executes a command with optional arguments and stdin input, bounded by a
/// timeout.
///
/// On Windows, bare program names (e.g. `main.exe`) are resolved relative to
/// the current directory by prepending `.\`. This mirrors the behaviour users
/// expect in a competitive-programming workflow.
///
/// # Errors
///
/// Returns [`ProcessError::ExecutionFailed`] if the process cannot be spawned
/// or its output cannot be read, and [`ProcessError::Timeout`] if it exceeds
/// `timeout_secs`.
pub fn execute_with_timeout(
    program: &str,
    args: &[&str],
    input: Option<&str>,
    timeout_secs: f64,
) -> Result<ProcessResult, ProcessError> {
    // Resolve bare filenames to the current directory on Windows so that
    // `main.exe` is found without requiring it to be on PATH.
    let resolved: Cow<'_, str> = if !program.contains('/') && !program.contains('\\') {
        let local = std::path::Path::new(program);
        if local.exists() {
            Cow::Owned(format!(".\\{program}"))
        } else {
            Cow::Borrowed(program)
        }
    } else {
        Cow::Borrowed(program)
    };

    let mut cmd = Command::new(resolved.as_ref());
    cmd.args(args);

    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| ProcessError::ExecutionFailed(format!("{program} - {e}")))?;

    // Feed stdin *before* waiting so the child can start processing immediately.
    if let Some(input_str) = input {
        if let Some(mut stdin) = child.stdin.take() {
            // Ignore BrokenPipe — the child may have closed stdin early.
            let _ = stdin.write_all(input_str.as_bytes());
        }
    }

    let timeout = Duration::from_secs_f64(timeout_secs);

    // Block on the child using an OS-native wait with a timeout.
    // This avoids the anti-pattern of polling `try_wait` in a busy loop.
    match child.wait_timeout(timeout) {
        Ok(Some(status)) => {
            let output = child
                .wait_with_output()
                .map_err(|e| ProcessError::ExecutionFailed(format!("Failed to read output: {e}")))?;

            Ok(ProcessResult {
                success: status.success(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                exit_code: status.code(),
            })
        }
        Ok(None) => {
            // Timed out — kill and reap the zombie process.
            let _ = child.kill();
            let _ = child.wait();
            Err(ProcessError::Timeout(timeout_secs))
        }
        Err(e) => Err(ProcessError::ExecutionFailed(e.to_string())),
    }
}
