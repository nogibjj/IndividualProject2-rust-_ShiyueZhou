# Rust Command-Line Utility for Multiplying Two Numbers

This project is a simple Rust-based command-line tool that multiplies two integers provided as input via the command line. It uses the `clap` library to parse command-line arguments and provides the result of the multiplication in a user-friendly format.

## Features
- Takes two integer inputs from the command line.
- Multiplies the two numbers and displays the result.
- Integrated with a CI/CD pipeline to ensure code quality, formatting, linting, and testing.

## How to Use

### Prerequisites
- Rust installed on your system. If you don't have it installed, you can get it [here](https://www.rust-lang.org/tools/install).

### Running the Program

1. Users provide two integers as input via the command line by running:
   ```bash
   cargo run -- num1 num2
    ```
2. The tool multiplies the two numbers and prints the result in a user-friendly format.
    **For example:**
   ```bash
   cargo run -- 1 2
    ```

    And the result will be:

    ```bash
      The multiplication of num1 and num2: 2
    ```
3. The project is integrated with a CI/CD pipeline to ensure code quality through automated testing and linting.