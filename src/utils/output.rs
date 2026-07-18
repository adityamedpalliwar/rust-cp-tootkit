use colored::{ColoredString, Colorize};

/// Prints an informational message to stdout.
pub fn info(msg: &str) {
    println!("{} {}", "ℹ".cyan().bold(), msg);
}

/// Prints a success message to stdout.
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Prints a warning message to stdout.
pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

/// Prints an error message to stderr.
pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Formats a duration nicely (e.g., "1.23s", "45ms").
pub fn format_duration(seconds: f64) -> ColoredString {
    if seconds < 0.001 {
        format!("{:.2}µs", seconds * 1_000_000.0).magenta()
    } else if seconds < 1.0 {
        format!("{:.2}ms", seconds * 1000.0).yellow()
    } else {
        format!("{:.2}s", seconds).red()
    }
}
