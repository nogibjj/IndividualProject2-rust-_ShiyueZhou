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
**<p style="font-size:20px;"> 1. Rust Command-Line Tool:</p>**

The main functionality is built around a command-line utility that multiplies two numbers provided as arguments.  
The tool uses the clap library to parse command-line inputs, allowing users to run the tool with **cargo run -- num1 num2**, where num1 and num2 are the integers to be multiplied.   


**<p style="font-size:20px;"> 2. Mathematical Operations:</p>**

A simple Rust function is used to multiply the two numbers and print the result in a user-friendly format. 
The utility showcases Rust's efficiency and strong type safety, ensuring correct handling of integer inputs. 

**<p style="font-size:20px;"> 3. Argument Parsing with clap:</p>**

The project uses the clap crate for parsing command-line arguments, making it easy for users to interact with the program by passing numbers directly through the terminal.
The arguments are handled in a structured way, ensuring the user input is validated and processed correctly.

**<p style="font-size:20px;"> 4. Makefile for Automation:</p>**

A Makefile is included to automate tasks such as running the program, testing, formatting code, and linting. This simplifies development and testing workflows.
The all target in the Makefile runs tasks such as formatting, linting, testing, and executing the program in sequence.

**<p style="font-size:20px;"> 5.CI/CD Pipeline: with binary file as an artifact in CI/CD</p>**

A CI/CD pipeline is integrated using GitHub Actions. This pipeline automates code formatting (cargo fmt), linting (cargo clippy), testing (cargo test), and building the project (cargo build).  
The pipeline ensures that the code meets quality standards every time changes are made. 
```plaintext
    - name: Upload Binary Artifact
      uses: actions/upload-artifact@v4
      with:
        name: MiniProject7-rust-_ShiyueZhou
        path: target/release/MiniProject7-rust-_ShiyueZhou
``` 

## Technologies Used:
* Rust: The core language used to implement the command-line tool.
* Clap: A Rust library for argument parsing.
* Cargo: Used for building, testing, and managing dependencies.
* Makefile: Automates common development tasks like running the program, formatting, and testing.
* GitHub Actions: For automating continuous integration and testing processes.

## User Guide:
[Contributing Guidelines](user_guide.md)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
