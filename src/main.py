# python_script.py

import csv
import math
import time
import tracemalloc
from collections import defaultdict
from statistics import mean, median, stdev

def is_number(s):
    """Check if the string can be converted to a float."""
    try:
        float(s)
        return True
    except ValueError:
        return False

def compute_statistics(data):
    """Compute mean, median, and standard deviation for numeric columns."""
    stats = {}
    for column, values in data.items():
        # Filter out non-numeric values
        numeric_values = [float(v) for v in values if is_number(v)]
        if numeric_values:
            if len(numeric_values) >= 2:
                stats[column] = {
                    'mean': mean(numeric_values),
                    'median': median(numeric_values),
                    'std_dev': stdev(numeric_values)
                }
            else:
                # If there's only one numeric value, stdev is not defined
                stats[column] = {
                    'mean': numeric_values[0],
                    'median': numeric_values[0],
                    'std_dev': float('nan')
                }
    return stats

def process_csv(file_path):
    # Start time and memory tracking
    start_time = time.time()
    tracemalloc.start()

    data = defaultdict(list)
    with open(file_path, 'r', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        headers = reader.fieldnames
        for row in reader:
            for header in headers:
                data[header].append(row[header])

    stats = compute_statistics(data)

    for column, stat in stats.items():
        print(f"Column: {column}")
        print(f"  Mean: {stat['mean']}")
        print(f"  Median: {stat['median']}")
        print(f"  Standard Deviation: {stat['std_dev']}")
        print()

    # Stop memory tracking and get memory usage
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    end_time = time.time()

    time_elapsed = end_time - start_time  # in seconds
    memory_used = peak / 10**6  # Convert bytes to megabytes

    print(f"Total Time Elapsed: {time_elapsed:.4f} seconds")
    print(f"Peak Memory Usage: {memory_used:.4f} MB")

if __name__ == "__main__":
    process_csv('data.csv')
