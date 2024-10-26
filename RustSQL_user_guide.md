# Rust ETL Command-Line Tool: User Guide

This Rust project rewrites a Python-based ETL (Extract, Transform, Load) script into a high-performance, resource-efficient command-line tool. It handles data extraction, transformation, loading, and querying, leveraging Rust's memory safety and concurrency for better scalability and performance.

## Project Structure

The project consists of two main files:
- **`main.rs`**: The entry point for executing the command-line program, handling argument parsing, resource tracking, and calling core functions.
- **`lib.rs`**: The core library module containing reusable functions for ETL operations, such as extracting data, loading into a database, and executing queries.

### File Descriptions

- **`src/main.rs`**: Handles command-line arguments, memory and time tracking, and initiates the ETL process. Uses the `tokio` runtime for asynchronous handling of I/O operations.
- **`src/lib.rs`**: Defines core functions grouped into modules (`extract`, `database`, and `load`) to encapsulate data processing tasks. These modules are called by `main.rs` to perform specific ETL operations.

## How to Use

### Prerequisites

- **Rust Installation**: Ensure Rust is installed. You can download it [here](https://www.rust-lang.org/tools/install).

### Running the Program

1. **Clone the Repository**: Clone the project repository and navigate into it.
   
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Run the Program:**: Use cargo run to start the program, specifying the --url and --file_path options for the data source URL and desired file path.
   
   ```bash
   cargo run -- --url <URL> --file_path <FILE_PATH>
   ```  

   Example:  
    ```bash
   cargo run -- --url "https://example.com/data.csv" --file_path "data/data.csv"
   ```  
3. **Output:**: The program downloads data from the specified URL, loads it into a local SQLite database, and runs predefined queries. The output will display the results, as well as memory usage and execution time.

## Examples of Command-Line Arguments
**Basic Usage:**
  ```bash
   cargo run -- --url "https://example.com/data.csv" --file_path "data/example.csv"

   ```  
Output example:  
  ```bash
  File successfully downloaded to data/example.csv
  Data successfully loaded into database.db
  Query executed successfully.
  Process completed in: 348.20ms
  Memory used: 4821 KB

   ```  

## Key Modules and Functions

### `lib.rs` Modules

- **`extract`**: Contains the `extract` function, which downloads data from a specified URL and saves it to a file. Uses `reqwest` for HTTP requests and `tokio` for async handling.
   ```rust
   pub async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>>
  ```
- **`load`**: Contains the `load` function, which reads a CSV file and loads its contents into an SQLite database. Uses `rusqlite` for database interactions and `csv` for CSV parsing
   ```rust
   pub fn load(dataset: &str) -> Result<(), Box<dyn Error>>
  ```
- **`database`**: Contains functions for database interactions:
    - **`connect_to_db`**: Connects to the SQLite database.
    - **`execute_query`**:  Executes predefined SQL queries.
   ```rust
   pub fn connect_to_db(db_path: &str) -> Result<Connection>
   pub fn execute_query(conn: &Connection, query: &str) -> Result<()>

  ```

  ### `main.rs` Functions
  - **`run_main_logic`**: Handles the entire ETL process, coordinating `extract`, `load`, and `execute_query` calls. Tracks execution time and memory usage with `sysinfo` for monitoring.
   ```rust
   async fn run_main_logic(args: &Args) -> Result<(), Box<dyn std::error::Error>>

  ```
- **Memory and Time Tracking**: Uses `sysinfo` to measure memory usage and `Instant` to calculate execution time for performance insights.