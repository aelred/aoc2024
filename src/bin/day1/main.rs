use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let (mut l_list, mut r_list) = parse();

    l_list.sort();
    r_list.sort();

    let total_diff = part1(&l_list, &r_list);
    println!("Part 1: {}", total_diff);

    let similarity_score = part2(l_list, r_list);
    println!("Part 2: {}", similarity_score);

    println!("Time: {}μs", start.elapsed().as_micros());
}

fn parse() -> (Vec<u64>, Vec<u64>) {
    let mut l_list = Vec::new();
    let mut r_list = Vec::new();

    for line in INPUT.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        l_list.push(u64::from_str(split[0]).unwrap());
        r_list.push(u64::from_str(split[1]).unwrap());
    }
    (l_list, r_list)
}

fn part1(l_list: &Vec<u64>, r_list: &Vec<u64>) -> u64 {
    let mut total_diff = 0;

    for (l_value, r_value) in l_list.iter().zip(r_list.iter()) {
        let diff = l_value.abs_diff(*r_value);
        total_diff += diff;
    }
    total_diff
}

fn part2(l_list: Vec<u64>, r_list: Vec<u64>) -> u64 {
    let mut similarity_score = 0;

    let mut l_iter = l_list.into_iter().peekable();
    let mut r_iter = r_list.into_iter().peekable();

    while let Some(value) = l_iter.next() {
        let mut l_count = 1;

        while l_iter.next_if_eq(&value).is_some() {
            l_count += 1;
        }

        while r_iter.next_if(|r| *r < value).is_some() {}

        let mut r_count = 0;
        while r_iter.next_if_eq(&value).is_some() {
            r_count += 1;
        }

        similarity_score += value * l_count * r_count;
    }
    similarity_score
}
