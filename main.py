"""
ETL-Query script with memory and runtime tracking
"""

import time
import psutil
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_data, read_data, update_data, delete_data


def track_memory_and_time(func, *args, func_name="Function"):
    """
    Tracks the memory usage and runtime of a function, and executes it.

    Args:
        func (callable): The function to track.
        *args: Arguments to pass to the function.
        func_name (str): Optional name of the function for logging.

    Returns:
        Any: The result of the function call.
    """
    # Memory and time tracking setup
    process = psutil.Process()
    start_time = time.time()
    start_memory = process.memory_info().rss / (1024 * 1024)  # Convert to MB

    print(f"Starting {func_name}...")
    result = func(*args)  # Execute the function and capture its result

    # Calculate time and memory usage
    end_time = time.time()
    end_memory = process.memory_info().rss / (1024 * 1024)  # Convert to MB
    time_taken = end_time - start_time
    memory_used = end_memory - start_memory

    # Print resource usage
    print(f"{func_name} completed in {time_taken:.2f} seconds.")
    print(f"Memory used by {func_name}: {memory_used:.2f} MB\n")

    return result  # Return the function result if needed


# ETL and Query operations with memory and time tracking
track_memory_and_time(extract, func_name="Data Extraction")
track_memory_and_time(load, func_name="Data Loading")

# Query operations
track_memory_and_time(create_data, func_name="Data Insertion")
track_memory_and_time(read_data, func_name="Data Reading")
track_memory_and_time(update_data, func_name="Data Update")
track_memory_and_time(delete_data, func_name="Data Deletion")
