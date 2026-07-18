//! Compares two output files for equality, ignoring trailing whitespace.

use crate::utils::{filesystem, output};
use anyhow::{bail, Result};

/// Reads two files and reports whether their contents match (modulo trailing
/// whitespace and empty lines).
pub fn execute(file1: &str, file2: &str) -> Result<()> {
    let content1 = filesystem::read_file(file1)?;
    let content2 = filesystem::read_file(file2)?;

    if compare_outputs(&content1, &content2) {
        output::success("Outputs match exactly (ignoring trailing whitespace).");
        Ok(())
    } else {
        output::error("Outputs differ.");
        bail!("Files {file1} and {file2} do not match.");
    }
}

/// Compares two strings line by line, ignoring trailing whitespace on each line
/// and filtering out blank lines.
///
/// This is the standard comparison strategy used in competitive programming
/// online judges.
pub fn compare_outputs(expected: &str, actual: &str) -> bool {
    let expected_lines: Vec<&str> = expected
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .collect();

    let actual_lines: Vec<&str> = actual
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .collect();

    expected_lines.len() == actual_lines.len()
        && expected_lines
            .iter()
            .zip(actual_lines.iter())
            .all(|(e, a)| e == a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_outputs_match() {
        assert!(compare_outputs("1 2 3\n", "1 2 3\n"));
    }

    #[test]
    fn trailing_whitespace_is_ignored() {
        assert!(compare_outputs("1 2 3 \n ", "1 2 3\n"));
    }

    #[test]
    fn multiline_outputs_match() {
        assert!(compare_outputs("1\n2\n", "1\n2\n"));
    }

    #[test]
    fn different_outputs_do_not_match() {
        assert!(!compare_outputs("1 2 3", "1 2 4"));
    }
}
