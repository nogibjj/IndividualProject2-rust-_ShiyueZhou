# Python ETL Command-Line Tool: User Guide

This project is a Python-based ETL (Extract, Transform, Load) script designed to perform data extraction, transformation, loading, and querying tasks while tracking memory usage and execution time. The script provides insights into resource usage during ETL processes, making it suitable for performance monitoring and optimization.

## Project Structure

The project includes two main components:

- **`main.py`**: Executes the ETL and query operations with memory and time tracking.
- **`test_main.py`**: Unit tests for each ETL operation, verifying the functionality of extraction, transformation, and query functions.

## How to Use

### Prerequisites

- **Python Installation**: Ensure Python is installed. You can download it from [here](https://www.python.org/downloads/).
- **Install Dependencies**: Install the required dependencies using `pip`. Run the following command in your project directory:

   ```bash
   pip install -r requirements.txt
   ```

Required dependencies:

- **`psutil`**: For monitoring memory usage.
- **`pytest`**: For running the test suite.

### Running the ETL Script

- **1. Execute the Script:**:  Run the `main.py` file to start the ETL process with memory and time tracking. This will execute each ETL operation sequentially and output the execution time and memory used for each function.
   ```bash
   python main.py

   ```
- **2. Output:**:  The script will display the time and memory used by each operation and a total summary at the end.

Example output:
   ```bash
   Data Loading completed in 0.02 seconds.
Memory used by Data Loading: 0.72 MB

Starting Data Insertion...
create:  [('city1', 'NC', '1', '2', '1')]
Data Insertion completed in 0.00 seconds.
Memory used by Data Insertion: 0.00 MB

Starting Data Reading...
read 10 from Murder2015
Data Reading completed in 0.00 seconds.
Memory used by Data Reading: 0.00 MB

Starting Data Update...
close
Data Update completed in 0.00 seconds.
Memory used by Data Update: 0.00 MB

Starting Data Deletion...
delete
Data Deletion completed in 0.00 seconds.
Memory used by Data Deletion: 0.00 MB


=== Total Summary ===
Total Execution Time: 0.16 seconds
Total Memory Used: 4.34 MB
   ```

### Testing
# Running Tests
The project includes unit tests for each ETL operation in `test_main.py`. These tests verify the success of data extraction, loading, and CRUD operations.

- **1. Execute the Test Suite:** Run `pytest` to execute the tests. Each function in `test_main.py` will be tested, and results will be printed to the console.
   ```bash
   pytest test_main.py -v

   ```
- **2. Expected Output:**   Each test should pass if the functions are working correctly.
 Example:  
   ```bash
   Read test passed.
close
Update test passed.
delete
Delete test passed.
All tests completed successfully.
   ``` 