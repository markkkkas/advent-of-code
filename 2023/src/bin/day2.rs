use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input/day2.txt";

    println!("{}", solve_part_one(path)?);

    Ok(())
}

fn solve_part_one(path: &str) -> io::Result<i32> {
    io::BufReader::new(File::open(path)?)
        .lines()
        .for_each(|f| println!("{}", f.unwrap_or_default()));

    Ok(0)
}
