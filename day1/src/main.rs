use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

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

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // Sort both arrays
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();

    // Calculate the sum of differences
    let sum_of_differences: i32 = left_sorted.iter()
        .zip(right_sorted.iter()) // Pair elements from both arrays
        .map(|(a, b)| ((*a as i32 - *b as i32).abs())) // Calculate absolute difference
        .sum(); // Sum all differences
    
    // Print the result inside the function
    println!("Total distance between lists: {}", sum_of_differences);

    // Return the result
    sum_of_differences
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // Count the occurrences of each number in the right list
    let mut right_counts = HashMap::new();
    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score: i32 = left.iter()
        .map(|&num| num * *right_counts.get(&num).unwrap_or(&0)) // Multiply the number by its count in the right list
        .sum(); // Sum the results

    // Print the result inside the function
    println!("Total similarity score: {}", similarity_score);

    // Return the result
    similarity_score
}

fn main() -> io::Result<()> {
    let file_path = "day1/input.txt"; // Specify the file path
    let contents = read_file(file_path)?; // Call the function to read the file

    let (left_numbers, right_numbers) = parse_numbers(&contents); // Parse the numbers into two arrays

    // Part 1: Calculate the sum of differences
    part1(&left_numbers, &right_numbers);

    // Part 2: Calculate the similarity score
    part2(&left_numbers, &right_numbers);

    Ok(())
}
