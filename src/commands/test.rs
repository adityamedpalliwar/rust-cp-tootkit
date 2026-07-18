//! Runs a compiled binary against sample test cases and compares outputs.

use crate::commands::compare::compare_outputs;
use crate::utils::{filesystem, output, process, timer::Timer};
use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

/// Discovers `.in`/`.out` test-case pairs in `tests_dir`, executes `binary`
/// with each input, and compares the output against the expected answer.
///
/// Test cases are matched by file stem: `sample1.in` pairs with `sample1.out`.
/// Cases without a corresponding `.out` file are silently skipped.
pub fn execute(binary: &str, tests_dir: &str, timeout: f64) -> Result<()> {
    output::info(&format!("Running tests from '{tests_dir}'..."));

    let dir_path = Path::new(tests_dir);
    if !dir_path.is_dir() {
        bail!("Tests directory '{tests_dir}' does not exist.");
    }

    // Collect matching .in/.out pairs using an idiomatic iterator chain.
    let mut test_cases: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "in"))
        .filter_map(|in_file| {
            let out_file = in_file.with_extension("out");
            out_file.exists().then_some((in_file, out_file))
        })
        .collect();

    if test_cases.is_empty() {
        output::warning("No test cases found. Ensure files are named *.in and *.out");
        return Ok(());
    }

    test_cases.sort();
    let total = test_cases.len();
    let mut passed = 0;

    for (in_file, out_file) in &test_cases {
        let test_name = in_file
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let input_content = filesystem::read_file(in_file)?;
        let expected_content = filesystem::read_file(out_file)?;

        let timer = Timer::start();
        let result = process::execute_with_timeout(binary, &[], Some(&input_content), timeout)?;
        let elapsed = timer.elapsed_secs();
        let duration = output::format_duration(elapsed);

        if !result.success {
            output::error(&format!("Test {test_name} - RUNTIME ERROR [{duration}]"));
            eprintln!("{}", result.stderr);
        } else if compare_outputs(&expected_content, &result.stdout) {
            output::success(&format!("Test {test_name} - ACCEPTED [{duration}]"));
            passed += 1;
        } else {
            output::error(&format!("Test {test_name} - WRONG ANSWER [{duration}]"));
            println!("Expected:\n{}", expected_content.trim());
            println!("Actual:\n{}", result.stdout.trim());
        }
    }

    if passed == total {
        output::success(&format!("All {total} tests passed!"));
    } else {
        output::warning(&format!("{passed}/{total} tests passed."));
    }

    Ok(())
}
