//! Scaffolds a new competitive-programming problem directory.

use crate::utils::{filesystem, output};
use anyhow::{Context, Result};
use std::path::Path;

/// Creates a new problem directory with a C++ template and a sample test case.
///
/// The resulting layout is:
/// ```text
/// <name>/
///   main.cpp
///   tests/
///     sample1.in
///     sample1.out
/// ```
pub fn execute(name: &str) -> Result<()> {
    output::info(&format!("Initializing project '{name}'..."));

    let base_path = Path::new(name);
    filesystem::create_dir_all(base_path).context("Failed to create project directory")?;

    let tests_path = base_path.join("tests");
    filesystem::create_dir_all(&tests_path).context("Failed to create tests directory")?;

    let main_cpp = base_path.join("main.cpp");
    let template = r#"#include <iostream>
#include <vector>

using namespace std;

void solve() {
    // Your code here
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    int t = 1;
    cin >> t;
    while (t--) {
        solve();
    }
    return 0;
}
"#;
    filesystem::write_file(&main_cpp, template).context("Failed to write main.cpp")?;

    // Create a dummy test case so `test` works out of the box.
    filesystem::write_file(tests_path.join("sample1.in"), "1\n")
        .context("Failed to write sample1.in")?;
    filesystem::write_file(tests_path.join("sample1.out"), "")
        .context("Failed to write sample1.out")?;

    output::success(&format!("Successfully initialized project in '{name}'"));
    Ok(())
}
