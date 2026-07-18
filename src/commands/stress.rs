//! Stress-tests two solutions against each other using generated inputs.

use crate::commands::compare::compare_outputs;
use crate::utils::{output, process};
use anyhow::{bail, Result};

/// Runs a stress test by repeatedly:
///
/// 1. Generating a random test case via `generator`.
/// 2. Running both `brute` (reference) and `optimized` (candidate) solutions.
/// 3. Comparing their outputs — stopping at the first mismatch.
///
/// If `iterations` is 0 the loop runs indefinitely until a mismatch is found
/// or the user interrupts with Ctrl-C.
pub fn execute(
    optimized: &str,
    brute: &str,
    generator: &str,
    iterations: usize,
    timeout: f64,
) -> Result<()> {
    output::info(&format!(
        "Starting stress test: opt={optimized}, brute={brute}, gen={generator}"
    ));

    for i in 1.. {
        if iterations > 0 && i > iterations {
            output::success(&format!(
                "Stress test completed. {iterations} iterations passed."
            ));
            break;
        }

        // 1. Generate test case
        let gen_result = process::execute_with_timeout(generator, &[], None, timeout)?;
        if !gen_result.success {
            output::error("Generator failed");
            eprintln!("{}", gen_result.stderr);
            bail!("Generator error");
        }
        let test_input = gen_result.stdout;

        // 2. Run brute-force reference solution
        let brute_result =
            process::execute_with_timeout(brute, &[], Some(&test_input), timeout)?;
        if !brute_result.success {
            output::error("Brute force solution failed");
            eprintln!("{}", brute_result.stderr);
            bail!("Brute force runtime error");
        }

        // 3. Run optimized candidate solution
        let opt_result =
            process::execute_with_timeout(optimized, &[], Some(&test_input), timeout)?;
        if !opt_result.success {
            output::error("Optimized solution failed");
            eprintln!("{}", opt_result.stderr);
            bail!("Optimized runtime error");
        }

        // 4. Compare outputs
        if compare_outputs(&brute_result.stdout, &opt_result.stdout) {
            if i % 10 == 0 {
                output::info(&format!("Passed {i} iterations..."));
            }
        } else {
            output::error(&format!("Mismatch found on iteration {i}"));
            println!("-- Input --\n{test_input}");
            println!("-- Expected (Brute) --\n{}", brute_result.stdout);
            println!("-- Actual (Optimized) --\n{}", opt_result.stdout);
            break;
        }
    }

    Ok(())
}
