# rust-new-project-template

[![Rust CI/CD Pipeline](https://github.com/nogibjj/MiniProject7-rust-_ShiyueZhou/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/MiniProject7-rust-_ShiyueZhou/actions/workflows/rust.yml)

## Summary
# Project Summary: Rust Command-Line Utility
This project is a Rust-based command-line tool designed to perform basic mathematical operations, specifically multiplying two numbers. Using the clap library, the project allows users to input two integers via the command line and receive the product of those numbers as the output. The focus of the project is on leveraging Rust for efficient, lightweight command-line utilities.  

# Project Main RUST Structure
```plaintext
.
├── src
│   └── main.rs      # The main Rust source file
├── Cargo.toml       # Rust project manifest
└── README.md        # Project documentation
```


# Key Components:
1. Command-Line Argument Parsing with clap:

The Args struct holds two command-line arguments, url and file_path, with default values for each.
The #[derive(Parser)] macro from clap allows parsing these arguments directly from the command line, or it falls back on the default values.

2. Refactoring with run_main_logic:

The main logic has been refactored into run_main_logic, making it testable.
This function accepts args and executes lib::extract, lib::execute_query, and lib::load functions.

3. Memory and Time Calculation:

The sysinfo crate tracks memory usage before and after run_main_logic, and the Instant type measures the elapsed time. This section should provide accurate memory usage and timing output for the process.

4. main Function Setup:

The main function uses tokio::main to handle asynchronous calls and calls run_main_logic with parsed command-line arguments.

5. Testing the Main Logic:

In the tests module, the test_run_main_logic function creates a mock Args struct with the actual murder data values.
This approach effectively tests run_main_logic in a controlled environment, simulating the command-line behavior.

## User Guide:
[Contributing Guidelines](user_guide.md)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
