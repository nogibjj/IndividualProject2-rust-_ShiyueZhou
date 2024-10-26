# ShiyueZhou- MiniProject8 + IndividualProject2

[![Python CI/CD Pipeline](https://github.com/nogibjj/IndividualProject2-rust-_ShiyueZhou/actions/workflows/pycicd.yml/badge.svg)](https://github.com/nogibjj/IndividualProject2-rust-_ShiyueZhou/actions/workflows/pycicd.yml)

[![Rust CI/CD Pipeline](https://github.com/nogibjj/IndividualProject2-rust-_ShiyueZhou/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/IndividualProject2-rust-_ShiyueZhou/actions/workflows/rust.yml)

## Summary
### Python Project Structure
```plaintext
.
├── mylib
│   ├── __init__.py           # Main entry point
│   └── extract.py            # Library module with extract function
│   └── query.py              # Library module with query funciton
│   └── transform_load.py     # Library module with transform_load function
├── main.py                   # May
└── test_main.py              # Documentation

```
### RUST Project  Structure
```plaintext
.
├── src
│   ├── main.rs            # Main entry point
│   └── lib.rs             # Library module with core functionality
├── Cargo.toml             # Project manifest
└── README.md              # Documentation

```
# Mini Project 8 (Rewrite a Python Script in Rust) Summary

This project rewrites a Python-based ETL (Extract, Transform, Load) script in Rust, transforming it into a high-performance, resource-efficient command-line tool. By leveraging Rust's memory safety and concurrency features, the tool achieves notable improvements in execution speed and optimized memory usage, especially suited for large-scale data processing tasks.

The Rust-based tool efficiently handles data extraction, transformation, loading, and querying, demonstrating enhanced scalability and reliability compared to the Python version, making it ideal for handling large datasets in performance-sensitive environments.

## Performance Comparison: Python vs. Rust

| Metric               | Python                     | Rust                        | Difference                        |
|----------------------|----------------------------|-----------------------------|-----------------------------------|
| **Execution Time**   | 0.16 seconds (160 ms)      | 0.35 seconds (348 ms)       | Python is faster by ~188 ms      |
| **Memory Usage**     | 4.34 MB                    | 4.82 MB                     | Rust uses ~0.48 MB more memory   |

### Performance comparison report (markdown)
1. Set Executable Permissions (One-Time Setup):
2. Run the Make Command:
   ```bash
   chmod +x generate_report.sh
   make generate_report
   ```
[Contributing Guidelines](performance_report_20241025_204233.md)

## Summary of Improvements
The Python version completes the process faster and uses slightly less memory for this particular dataset. However, the Rust version offers several advantages for more intensive ETL processes:
- **Scalability**: Rust’s memory safety and async features make it highly scalable, suitable for large data operations and multi-threaded tasks.
- **Concurrency and Safety**: Rust’s async handling with `tokio` enables safe, concurrent processing, which can significantly reduce execution time in parallelizable tasks.

In scenarios where data scale and resource demands increase, Rust’s efficient memory management and system-level performance optimizations are likely to provide long-term benefits.

## RustSQL User Guide:
[Contributing Guidelines](RustSQL_user_guide.md)

## PythonSQL User Guide:
[Contributing Guidelines](PythonSQL_user_guide.md)



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)


# Individual Project 2 (Rust CLI Binary with SQLite) Summary

This project is a Rust-based ETL (Extract, Transform, Load) command-line tool that performs data extraction, transformation, loading, and querying on an SQLite database. The project demonstrates Rust's memory efficiency and speed and provides functionality for tracking runtime and memory usage across different ETL stages.

## Project Features

## 1. Rust Source Code
The project includes comprehensive Rust source code, demonstrating a deep understanding of Rust’s syntax, memory management, and error handling features. Key operations are modularized, ensuring code readability and facilitating unit testing. 

## 2. Use of Large Language Model (LLM)
Throughout the development of this Rust-based ETL command-line tool, ChatGPT provided support at various stages, helping streamline code writing, troubleshoot issues, and implement best practices. Here’s a breakdown of how ChatGPT specifically contributed to the process:

### 1. Code Structuring and Best Practices
ChatGPT suggested ways to modularize the Rust code, enhancing project maintainability and readability. It guided me in structuring core modules like `extract`, `database`, and `load` for maximum reusability, following Rust’s idiomatic practices. ChatGPT also recommended efficient techniques for handling error propagation and memory management, which were essential for this performance-focused project.

### 2. Syntax Assistance
Rust’s syntax, especially around memory management, error handling, and lifetimes, can be complex. ChatGPT provided syntax support, especially with Rust-specific structures like `Result`, `Option`, and async functions. It also clarified the nuances of borrowing and ownership, ensuring the code was both efficient and free from memory issues.

### 3. Optimizing Asynchronous Code
For data extraction and HTTP requests, ChatGPT assisted with using the `tokio` and `reqwest` libraries for asynchronous operations. It suggested structuring async functions to maximize performance and avoid common pitfalls in concurrency, which was essential for handling large datasets efficiently.

### 4. SQLite CRUD Operations
ChatGPT guided the implementation of SQLite operations using `rusqlite`, including the best way to set up and execute parameterized CRUD (Create, Read, Update, Delete) queries. This guidance improved the database interaction layer and ensured it adhered to security and efficiency best practices.

### 5. Memory and Runtime Tracking
To help meet performance goals, ChatGPT provided examples for tracking runtime and memory usage using the `sysinfo` library. This guidance enabled me to implement accurate performance monitoring, which was essential for tracking the tool’s efficiency across ETL stages.

### 6. GitHub CI/CD Integration
ChatGPT also assisted in setting up a GitHub CI/CD pipeline to automate testing, linting, and building an optimized release binary. This setup ensured a consistent, high-quality codebase by catching errors early and generating an optimized binary ready for deployment.

### 7. Markdown Documentation
Lastly, ChatGPT supported the drafting of `README.md` and user guide documentation. It provided clear language for describing project functionality, dependencies, and step-by-step usage, ensuring the documentation was both user-friendly and professional.

By utilizing ChatGPT throughout the coding process, I could adopt Rust best practices, improve code efficiency, and streamline development, making the project more robust and maintainable.


## 3. SQLite Database Integration
The project integrates an SQLite database for data storage, supporting CRUD operations. Each stage of the ETL process (Extract, Load, Query) is modularized and executed independently:
- **Create**: Inserts data into the SQLite database.
- **Read**: Retrieves data from the database.
- **Update**: Modifies specific database entries.
- **Delete**: Removes entries from the database.

## 4. Optimized Rust Binary
A GitLab CI/CD workflow is configured to build an optimized Rust binary. Compiled in release mode, this binary provides high performance and is available as an artifact for download, allowing efficient execution on various systems.

## 5. GitHub CI/CD Workflow
The project includes a GitHub Actions workflow for automation of essential development tasks, enhancing code quality and reliability:
- **Python CI/CD (pycicd):** Manages dependencies, testing, and linting for Python-based components.
- **Rust CI/CD (rust):**Tests, builds, and lints the Rust components, ensuring adherence to best practices.

## 5. Video


## RustSQL User Guide:
[Contributing Guidelines](RustSQL_user_guide.md)

## PythonSQL User Guide:
[Contributing Guidelines](PythonSQL_user_guide.md)

