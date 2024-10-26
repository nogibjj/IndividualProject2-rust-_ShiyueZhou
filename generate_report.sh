#!/bin/bash

# Create a timestamped report filename
report="performance_report_$(date +'%Y%m%d_%H%M%S').md"

# Run the Python script and capture performance output
echo "Running Python script..."
python_output=$(python main.py)

# Run the Rust script and capture performance output
echo "Running Rust script..."
rust_output=$(cargo run --release | tail -n +2)  # Strip build info

# Extract Python performance metrics
python_time=$(echo "$python_output" | grep "Total Execution Time" | awk '{print $4}')
python_memory=$(echo "$python_output" | grep "Total Memory Used" | awk '{print $4, $5}')

# Extract Rust performance metrics
rust_time=$(echo "$rust_output" | grep "Process completed in" | awk '{print $4}')
rust_memory=$(echo "$rust_output" | grep "Memory used" | awk '{print $3, $4}')

# Generate Markdown report
echo "# Performance Report" > $report
echo "" >> $report
echo "Generated on $(date)" >> $report
echo "" >> $report
echo "## Python Script Performance" >> $report
echo "" >> $report
echo "\`\`\`" >> $report
echo "$python_output" >> $report
echo "\`\`\`" >> $report
echo "" >> $report
echo "- **Execution Time**: $python_time seconds" >> $report
echo "- **Memory Usage**: $python_memory" >> $report
echo "" >> $report
echo "## Rust Script Performance" >> $report
echo "" >> $report
echo "\`\`\`" >> $report
echo "$rust_output" >> $report
echo "\`\`\`" >> $report
echo "" >> $report
echo "- **Execution Time**: $rust_time" >> $report
echo "- **Memory Usage**: $rust_memory" >> $report

echo "Performance report generated in $report"
