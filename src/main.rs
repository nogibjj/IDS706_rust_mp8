// src/main.rs

use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::time::{Duration, Instant};

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn compute_statistics(values: &[f64]) -> (f64, f64, f64) {
    let n = values.len();
    let mean = values.iter().sum::<f64>() / n as f64;

    let mut sorted_values = values.to_vec();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if n % 2 == 0 {
        let mid = n / 2;
        (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
    } else {
        sorted_values[n / 2]
    };

    let variance = values
        .iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f64>()
        / (n as f64 - 1.0);

    let std_dev = variance.sqrt();

    (mean, median, std_dev)
}

fn get_memory_usage_kb() -> u64 {
    // Returns the current memory usage of the process in kilobytes
    // Works on Unix-like systems
    let file = File::open("/proc/self/status");
    if let Ok(mut file) = file {
        use std::io::Read;
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(mem_kb) = parts[1].parse::<u64>() {
                            return mem_kb;
                        }
                    }
                }
            }
        }
    }
    0
}

fn process_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Start time and memory tracking
    let start_time = Instant::now();
    let start_memory = get_memory_usage_kb();

    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    let headers = rdr.headers()?.clone();
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for header in headers.iter() {
        data.insert(header.to_string(), Vec::new());
    }

    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            let header_name = &headers[i];
            data.get_mut(header_name).unwrap().push(field.to_string());
        }
    }

    for (column, values) in data.iter() {
        let numeric_values: Vec<f64> = values
            .iter()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect();

        if !numeric_values.is_empty() {
            if numeric_values.len() >= 2 {
                let (mean, median, std_dev) = compute_statistics(&numeric_values);
                println!("Column: {}", column);
                println!("  Mean: {}", mean);
                println!("  Median: {}", median);
                println!("  Standard Deviation: {}", std_dev);
                println!();
            } else if numeric_values.len() == 1 {
                let value = numeric_values[0];
                println!("Column: {}", column);
                println!("  Mean: {}", value);
                println!("  Median: {}", value);
                println!("  Standard Deviation: Undefined (only one value)");
                println!();
            }
        }
    }

    // End time and memory tracking
    let duration = start_time.elapsed();
    let end_memory = get_memory_usage_kb();
    let memory_used = end_memory as i64 - start_memory as i64; // in kilobytes

    println!("Total Time Elapsed: {:.4} seconds", duration.as_secs_f64());
    println!("Memory Used: {} KB", memory_used);

    Ok(())
}

fn main() {
    if let Err(err) = process_csv("data.csv") {
        eprintln!("Error processing CSV file: {}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::File;
    use csv::Reader;

    #[test]
    fn test_compute_statistics() {
        // Read the CSV file
        let file = File::open("data.csv").expect("Cannot open data.csv");
        let mut rdr = Reader::from_reader(file);

        let headers = rdr.headers().expect("Cannot read headers").clone();
        let mut data: HashMap<String, Vec<String>> = HashMap::new();

        for header in headers.iter() {
            data.insert(header.to_string(), Vec::new());
        }

        for result in rdr.records() {
            let record = result.expect("Cannot read record");
            for (i, field) in record.iter().enumerate() {
                let header_name = &headers[i];
                data.get_mut(header_name).unwrap().push(field.to_string());
            }
        }

        // Compute statistics for the 'Age' and 'G' columns
        let age_values: Vec<f64> = data["Age"]
            .iter()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect();
        let g_values: Vec<f64> = data["G"]
            .iter()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect();

        let (age_mean, age_median, _age_std_dev) = compute_statistics(&age_values);
        let (g_mean, g_median, _g_std_dev) = compute_statistics(&g_values);

        // Perform assertions
        assert!(age_values.len() > 0);
        assert_eq!(age_median, 25.0);
        assert_eq!(g_median, 37.0);

        // Additional assertions can be added as needed
    }
}