use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rust-cp-toolkit",
    version,
    about = "A feature-rich Rust CLI for competitive programming",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate project folder structure
    Init {
        /// Name of the problem or directory
        name: String,
    },
    /// Compile C++ solution using g++
    Compile {
        /// The C++ source file to compile
        file: String,

        /// Output binary name (optional)
        #[arg(short, long)]
        output: Option<String>,

        /// C++ standard version (e.g., c++17, c++20)
        #[arg(long, default_value = "c++17")]
        cpp_version: String,

        /// Apply optimization flag (-O2)
        #[arg(short = 'O', long, default_value_t = true)]
        optimize: bool,
    },
    /// Run executable
    Run {
        /// The binary to run
        binary: String,

        /// Arguments to pass to the binary
        #[arg(num_args = 0..)]
        args: Vec<String>,
    },
    /// Run against sample test cases
    Test {
        /// The binary to test
        binary: String,

        /// Directory containing test cases (inputs and expected outputs)
        #[arg(short = 'd', long, default_value = "./tests")]
        tests_dir: String,

        /// Timeout per test case in seconds
        #[arg(short, long, default_value_t = 2.0)]
        timeout: f64,
    },
    /// Run stress testing repeatedly
    Stress {
        /// Optimized solution binary
        #[arg(short, long)]
        optimized: String,

        /// Brute force / naive solution binary
        #[arg(short, long)]
        brute: String,

        /// Test case generator binary
        #[arg(short, long)]
        generator: String,

        /// Number of iterations to run (0 for infinite)
        #[arg(short, long, default_value_t = 0)]
        iterations: usize,

        /// Timeout per execution in seconds
        #[arg(short, long, default_value_t = 2.0)]
        timeout: f64,
    },
    /// Measure execution time
    Benchmark {
        /// The binary to benchmark
        binary: String,

        /// Input file to feed to standard input
        #[arg(short, long)]
        input: Option<String>,

        /// Arguments to pass to the binary
        #[arg(num_args = 0..)]
        args: Vec<String>,

        /// Number of times to run
        #[arg(short, long, default_value_t = 10)]
        runs: usize,
    },
    /// Compare expected vs actual output
    Compare {
        /// First file
        file1: String,

        /// Second file
        file2: String,
    },
    /// Remove generated files
    Clean,
}
