use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";

    println!("Part 1: {}", solve_part_one(path)?);
    println!("Part 2: {}", solve_part_two(path)?);

    Ok(())
}

fn calibrate(line: String) -> (i32, i32) {
    let mut left: i32 = -1;
    let mut right: i32 = -1;

    for char in line.chars() {
        if char.is_digit(10) {
            if left == -1 {
                left = char.to_digit(10).unwrap() as i32;
                right = char.to_digit(10).unwrap() as i32;
            } else {
                right = char.to_digit(10).unwrap() as i32;
            }
        }
    }

    (left, right)
}

fn solve_part_one(path: &str) -> io::Result<i32> {
    let mut sum = 0;

    for line in io::BufReader::new(File::open(path)?).lines() {
        let (right, left) = calibrate(line?);

        sum += format!("{}{}", right, left).parse::<i32>().unwrap();
    }

    Ok(sum)
}

fn solve_part_two(path: &str) -> io::Result<i32> {
    let m = HashMap::from([
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
    let mut sum = 0;

    for line_result in io::BufReader::new(File::open(path)?).lines() {
        let mut line = line_result?;

        for (key, value) in m.iter() {
            line = line.replace(key, &value);
        }

        let (right, left) = calibrate(line);

        sum += format!("{}{}", right, left).parse::<i32>().unwrap();
    }

    Ok(sum)
}
