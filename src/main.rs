mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Route to the appropriate command handler
    match cli.command {
        Commands::Init { name } => commands::init::execute(&name),
        Commands::Compile { file, output, cpp_version, optimize } => {
            commands::compile::execute(&file, output.as_deref(), &cpp_version, optimize)
        }
        Commands::Run { binary, args } => commands::run::execute(&binary, &args),
        Commands::Test { binary, tests_dir, timeout } => {
            commands::test::execute(&binary, &tests_dir, timeout)
        }
        Commands::Stress { optimized, brute, generator, iterations, timeout } => {
            commands::stress::execute(&optimized, &brute, &generator, iterations, timeout)
        }
        Commands::Benchmark { binary, input, args, runs } => {
            commands::benchmark::execute(&binary, input.as_deref(), &args, runs)
        }
        Commands::Compare { file1, file2 } => commands::compare::execute(&file1, &file2),
        Commands::Clean => commands::clean::execute(),
    }
}
