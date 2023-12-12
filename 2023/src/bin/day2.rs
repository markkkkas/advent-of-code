use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input/day2.txt";

    println!("Part 1: {}", solve_part_one(path)?);
    println!("Part 2: {}", solve_part_two(path)?);

    Ok(())
}

fn solve_part_one(path: &str) -> io::Result<i32> {
    let sum = io::BufReader::new(File::open(path)?)
        .lines()
        .enumerate()
        .filter_map(|(i, line)| line.ok().map(|l| (i as i32 + 1, l)))
        .filter(|(_, line)| {
            line.split(": ").nth(1).map_or(false, |reveals| {
                reveals.split("; ").all(|reveal| {
                    reveal.split(", ").all(|cube| {
                        cube.split_once(" ").map_or(false, |(count, color)| {
                            let count = count.parse::<i32>().unwrap_or(0);
                            match color {
                                "red" if count > 12 => false,
                                "green" if count > 13 => false,
                                "blue" if count > 14 => false,
                                _ => true,
                            }
                        })
                    })
                })
            })
        })
        .map(|(i, _)| i)
        .sum();

    Ok(sum)
}

fn solve_part_two(path: &str) -> io::Result<i32> {
    let sum = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(": ").nth(1).map_or(0, |reveals| {
                let (max_red, max_green, max_blue) = reveals
                    .split("; ")
                    .flat_map(|reveal| reveal.split(", "))
                    .filter_map(|cube| cube.split_once(" "))
                    .fold(
                        (0, 0, 0),
                        |(max_red, max_green, max_blue), (count, color)| {
                            let count = count.parse::<i32>().unwrap_or(0);
                            match color {
                                "red" if count > max_red => (count, max_green, max_blue),
                                "green" if count > max_green => (max_red, count, max_blue),
                                "blue" if count > max_blue => (max_red, max_green, count),
                                _ => (max_red, max_green, max_blue),
                            }
                        },
                    );

                max_red * max_green * max_blue
            })
        })
        .sum();

    Ok(sum)
}
