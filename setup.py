# pip install setuptools
from setuptools import setup, find_packages

setup(
    name="library",
    version="0.1",
    packages=find_packages(),
    entry_points={
        "console_scripts":["calculator=main:main"],
    }
)