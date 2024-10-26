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
## Mini Project 8 (Rewrite a Python Script in Rust) Summary

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

## Python User Guide:
[Contributing Guidelines](PythonSQL_user_guide.md)



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
