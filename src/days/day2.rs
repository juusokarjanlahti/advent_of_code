use std::{fs::File, io::{ BufRead, BufReader}, io::Write};

fn is_safe_report(report: &Vec<i32>) -> bool {
    for pair in report.windows(2) {
        let diff = pair[1] - pair[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false; // Unsafe if the difference isn't in the allowed range
        }
        if diff == 0 {
            return false; // Unsafe if numbers are the same
        }
    }
    true // All checks passed
}

pub fn run() {
    let input_path = "input2.csv";
    let output_path = "result2.csv";

    // Read the input CSV file.
    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let rows: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok()) // Skip lines that can't be read
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .map(|line| {
            let row: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();
            println!("Parsed row: {:?}", row);
            row
        })
        .filter(|row| !row.is_empty()) // Ensure no empty rows are included
        .collect();

    let mut safe_count = 0;
    for report in rows {
        if is_safe_report(&report) {
            if report.len() < 2 {
                safe_count += 0; // Reports with fewer than 2 numbers cannot be safe
            }
            safe_count += 1;
        }
    }

    // Write the result to the output CSV file.
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    writeln!(output_file, "Number of safe reports: {}", safe_count).expect("Failed to write to output file");

    println!("Number of safe reports: {}", safe_count);
}