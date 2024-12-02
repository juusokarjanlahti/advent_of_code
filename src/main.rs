use std::{fs::File, io::Read};

fn main() {
    // Attempt to open the input CSV file.
    let file: Result<File, std::io::Error> = File::open("input.csv");

    // Handle the Result of opening the file.
    let mut file: File = match file {
        Ok(file) => file,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };

    // Attempt to read the contents of the file.
    let mut contents: String = String::new();

    // Handle the Result of reading the file.
    match file.read_to_string(&mut contents) {
        Ok(file) => file,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };

    // Create vectors to store the number on the left and right of the CSV file.
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // Populate the vectors by splitting the lines of the CSV file and parsing the numbers from each line.
    for line in contents.lines() {
        let mut fields = line.split_whitespace();
        left_numbers.push(fields.next().unwrap().trim().parse().unwrap());
        right_numbers.push(fields.next().unwrap().trim().parse().unwrap());
    }

    let mut similarity_score: i32 = 0;
    // Iterate over the elements of the left_numbers vector.
    for &left in &left_numbers {
        // Create an iterator over the elements of the right_numbers vector.
        // Use the filter method to keep only elements equal to the left number.
        // The closure (|&&x| x == left) checks if each element is equal to the left number.
        // The count method counts how many elements match the left number.
        // Convert the count from usize to i32.
        let count = right_numbers.iter().filter(|&&x| x == left).count() as i32;
        similarity_score += left * count;
    }

    // Sort both lists.
    left_numbers.sort();
    right_numbers.sort();

    // Calculate the total distance.
    let mut total_distance: i32 = 0;
    for i in 0..left_numbers.len() {
        let left = left_numbers[i];
        let right = right_numbers[i];
        let difference = (left - right).abs();
        total_distance += difference;
    }

    // Create the output file.
    let output_file_result = File::create("result.csv");
    
    // Handle the Result of creating the file.
    let output_file = match output_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to create output file: {:?}", error);
            return;
        }
    };

    // Open the output CSV file.
    let mut wtr = csv::Writer::from_writer(output_file);

    // Write the similarity score to the output CSV file.
    wtr.write_record(&["Similarity Score", &similarity_score.to_string()]).unwrap();

    // Write the total distance to the output CSV file.
    wtr.write_record(&["Total Distance", &total_distance.to_string()]).unwrap();

    // Flush the writer to ensure all data is written.
    wtr.flush().unwrap();

    println!("Similarity score and total distance written to result.csv based on the input.csv.");
}