//! Benchmarks a compiled binary by running it multiple times and reporting
//! timing statistics.

use crate::utils::{filesystem, output, process, timer::Timer};
use anyhow::{bail, Result};

/// Runs `binary` for `runs` iterations and reports min / max / average
/// execution time.
///
/// If an `input` file path is provided, its contents are fed to the binary's
/// stdin on every iteration. This is essential for competitive-programming
/// solutions that read from stdin.
pub fn execute(binary: &str, input: Option<&str>, args: &[String], runs: usize) -> Result<()> {
    if runs == 0 {
        bail!("Number of runs must be > 0");
    }

    output::info(&format!("Benchmarking {binary} over {runs} runs..."));

    let input_content = input.map(filesystem::read_file).transpose()?;
    let args_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    let mut times = Vec::with_capacity(runs);

    for i in 1..=runs {
        let timer = Timer::start();
        let result = process::execute_with_timeout(
            binary,
            &args_refs,
            input_content.as_deref(),
            60.0,
        )?;
        let elapsed = timer.elapsed_secs();

        if !result.success {
            output::error(&format!("Run {i} failed"));
            bail!("Benchmark aborted due to runtime error");
        }

        times.push(elapsed);
    }

    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let min = times[0];
    let max = times[times.len() - 1];
    let avg: f64 = times.iter().sum::<f64>() / runs as f64;

    output::success("Benchmark completed");
    println!("Min: {}", output::format_duration(min));
    println!("Max: {}", output::format_duration(max));
    println!("Avg: {}", output::format_duration(avg));

    Ok(())
}
