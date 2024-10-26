"""
Unit tests for ETL and query operations.
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_data, read_data, update_data, delete_data


def test_extract():
    result = extract()
    assert result == "data/murder_2015_final.csv", "Extract test failed!"
    print("Extract test passed.")


def test_transform():
    result = load()
    assert result == "Murder2015.db", "Transform test failed!"
    print("Transform test passed.")


def test_create():
    result = create_data()
    assert result == "Create Success", "Create test failed!"
    print("Create test passed.")


def test_read():
    result = read_data()
    assert result == "Read Success", "Read test failed!"
    print("Read test passed.")


def test_update():
    result = update_data()
    assert result == "Update Success", "Update test failed!"
    print("Update test passed.")


def test_delete():
    result = delete_data()
    assert result == "Delete Success", "Delete test failed!"
    print("Delete test passed.")


if __name__ == "__main__":
    # Run all tests
    test_extract()
    test_transform()
    test_create()
    test_read()
    test_update()
    test_delete()
    print("All tests completed successfully.")
