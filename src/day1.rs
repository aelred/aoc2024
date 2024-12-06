use aoc_runner_derive::aoc;
use std::str::FromStr;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut l_list = Vec::new();
    let mut r_list = Vec::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        l_list.push(u64::from_str(split[0]).unwrap());
        r_list.push(u64::from_str(split[1]).unwrap());
    }
    (l_list, r_list)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let (l_list, r_list) = parse(input);

    let mut total_diff = 0;

    for (l_value, r_value) in l_list.into_iter().zip(r_list.iter()) {
        let diff = l_value.abs_diff(*r_value);
        total_diff += diff;
    }

    total_diff
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let (l_list, r_list) = parse(input);

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
