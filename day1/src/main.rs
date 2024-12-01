use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?; // Open the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read the file contents into a string
    Ok(contents)
}

fn parse_numbers(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    // Split contents into lines and process each line
    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by spaces/tabs
            .filter_map(|num| num.parse().ok()) // Convert to i32, ignore invalid lines
            .collect();

        if numbers.len() == 2 { // Ensure each line has exactly two numbers
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    (left_numbers, right_numbers)
}

fn main() -> io::Result<()> {
    let file_path = "day1/input.txt"; // Specify the file path
    let contents = read_file(file_path)?; // Call the function to read the file

    let (mut column1, mut column2) = parse_numbers(&contents); // Parse the numbers into two arrays

    // Sort both arrays
    column1.sort();
    column2.sort();

    // Calculate the sum of differences
    let sum_of_differences: i32 = column1.iter()
        .zip(column2.iter()) // Pair elements from both arrays
        .map(|(a, b)| ((*a - *b).abs())) // Calculate absolute difference
        .sum(); // Sum all differences
    
    // Print the result
    println!("Total distance between lists: {}", sum_of_differences);

    Ok(())
}
