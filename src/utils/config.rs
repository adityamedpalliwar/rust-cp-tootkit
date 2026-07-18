/// Configuration constants for the toolkit
pub const DEFAULT_OPTIMIZATION_FLAG: &str = "-O2";
pub const DEFAULT_WARNING_FLAGS: &[&str] = &["-Wall", "-Wextra", "-Wshadow", "-Wconversion"];

/// Returns the default compilation arguments for g++.
pub fn get_default_compile_args(
    input_file: &str,
    output_file: &str,
    cpp_version: &str,
    optimize: bool,
) -> Vec<String> {
    let mut args = vec![
        input_file.to_string(),
        "-o".to_string(),
        output_file.to_string(),
        format!("-std={}", cpp_version),
    ];
    
    for &flag in DEFAULT_WARNING_FLAGS {
        args.push(flag.to_string());
    }

    if optimize {
        args.push(DEFAULT_OPTIMIZATION_FLAG.to_string());
    }

    args
}
