use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";

    println!("Part 1: {}", solve_part_one(path)?);
    println!("Part 2: {}", solve_part_two(path)?);

    Ok(())
}

fn calibrate(line: String) -> i32 {
    let mut left = '-';
    let mut right = '-';

    for char in line.chars() {
        if char.is_digit(10) {
            if left == '-' {
                left = char;
                right = char;
            } else {
                right = char;
            }
        }
    }

    format!("{}{}", left, right).parse::<i32>().unwrap()
}

fn solve_part_one(path: &str) -> io::Result<i32> {
    let sum: i32 = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|line| calibrate(line.unwrap_or_default()))
        .sum();

    Ok(sum)
}

fn solve_part_two(path: &str) -> io::Result<i32> {
    let replacements = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let sum: i32 = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|line_result| {
            let line = line_result.unwrap_or_default();
            replacements
                .iter()
                .fold(line, |acc, (key, value)| acc.replace(key, value))
        })
        .map(|line| calibrate(line))
        .sum();

    Ok(sum)
}
