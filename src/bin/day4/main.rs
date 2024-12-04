use std::fmt::Display;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let _ = parse();

    println!("Part 1: {}", part1());

    println!("Part 2: {}", part2());

    println!("Time: {}μs", start.elapsed().as_micros());
}

fn parse() -> Vec<Vec<char>> {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}

fn part1() -> impl Display {
    "TODO".to_owned()
}

fn part2() -> impl Display {
    "TODO".to_owned()
}
