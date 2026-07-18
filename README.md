# Rust CP Toolkit (`rust-cp-toolkit`)

A Rust command-line toolkit that streamlines compiling, testing, benchmarking, and stress-testing competitive programming solutions.

---

## Motivation

When solving competitive programming problems, developers repeatedly perform the same workflow:

- Create a problem directory
- Write a solution
- Compile it
- Run sample tests
- Measure execution time
- Stress-test against randomized cases

This project automates that workflow through a single Rust-based command-line application, reducing repetitive tasks and providing a consistent development experience.

---

## Project Overview

**Welcome!** If you are reviewing this repository for a Software Engineering role, this project was built from scratch to demonstrate my proficiency in systems programming and my ability to write idiomatic, production-ready Rust code.

###  Rust Concepts Demonstrated:
- **Ownership & Borrowing:** Extensive use of references (`&str`, `&[String]`) to pass data between CLI routers and command executors without unnecessary memory allocations or `.clone()` calls.
- **Robust Error Handling:** Zero use of `.unwrap()` in production code. Uses `anyhow` for top-level application context and `thiserror` for domain-specific library errors (`FsError`, `ProcessError`).
- **Modular Architecture:** Clean separation of concerns. The CLI parsing layer (`src/cli.rs`) is isolated from the business logic (`src/commands/`), which in turn relies on reusable, stateless utility modules (`src/utils/`).
- **Concurrency & Process Management:** Safe execution of child processes (`std::process::Command`) with custom thread-based timeouts to prevent hanging on infinite loops.
- **Testing:** Integration tests using `assert_cmd` and `predicates` to verify end-to-end CLI behavior.

---

##  How to Test This Project in 60 Seconds

I've made it as easy as possible to verify that this toolkit works locally on your machine.

**Prerequisites:** You will need [Rust](https://rustup.rs/) and a C++ compiler (`g++` or `msvc`) installed.

1. **Clone & Build**
   ```bash
   git clone https://github.com/adityamedpalliwar/rust-cp-toolkit.git
   cd rust-cp-toolkit
   cargo build
   ```

2. **Initialize a Dummy Problem**
   ```bash
   cargo run -- init problem_a
   cd problem_a
   ```
   *(This instantly generates a `main.cpp` template and a `tests/` folder with dummy inputs/outputs).*

3. **Compile & Test the Solution**
   ```bash
   cargo run -- compile main.cpp
   cargo run -- test main.exe
   ```
   *(Colored terminal output indicating passed and failed test cases.).*

4. **Benchmark the Execution Speed**
   ```bash
   cargo run -- benchmark main.exe --input tests/sample1.in --runs 10
   ```
   *(Measures the execution time across multiple runs of the binary).*

---

## Features

- Initialize a competitive programming workspace
- Compile C++ solutions using configurable compiler flags
- Execute compiled programs safely with timeout protection
- Validate solutions against sample test cases
- Compare program output with expected output
- Benchmark execution time across multiple runs
- Stress-test solutions using generated test cases
- Clean generated artifacts

---

## 📂 Architecture & Project Structure

```text
rust-cp-toolkit/
├── src/
│   ├── main.rs            # Entry point & Command Router
│   ├── cli.rs             # Clap CLI definitions (Argument Parsing)
│   ├── commands/          # Subcommands (Business Logic)
│   │   ├── init.rs
│   │   ├── compile.rs
│   │   ├── run.rs
│   │   ├── test.rs
│   │   ├── stress.rs
│   │   ├── benchmark.rs
│   │   ├── compare.rs
│   │   └── clean.rs
│   └── utils/             # Reusable Domain Modules
│       ├── filesystem.rs  # Safe I/O abstractions
│       ├── process.rs     # Process execution with timeouts
│       ├── timer.rs       # High-resolution time measurement
│       └── output.rs      # Colored terminal formatting
├── tests/                 
│   └── integration_test.rs # End-to-End automated CLI tests
└── Cargo.toml             # Dependencies (clap, anyhow, thiserror, colored)
```

## 🧠 Design Decisions

- **Why `clap`?** Chosen for its robust derive macros which make the CLI self-documenting, type-safe, and incredibly easy to maintain.
- **Dependency Minimization:** Relied heavily on the Rust Standard Library (e.g., standard `std::process`, custom thread-based timeouts) to minimize bloat and keep compile times under a few seconds. 
- **Error Handling:** The toolkit never panics if a user provides a bad file or if a C++ program infinite-loops. Errors are caught, formatted nicely in red text, and returned to the user safely.
