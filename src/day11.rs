use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    run(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    run(input, 75)
}

fn run(input: &str, count: u64) -> u64 {
    let stones: Stones = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut memory = HashMap::new();

    let mut result = 0;

    for stone in stones {
        result += memo(&mut memory, stone, count);
    }

    result
}

fn memo(memory: &mut HashMap<(u64, u64), u64>, stone: u64, count: u64) -> u64 {
    if count == 0 {
        return 1;
    }

    if let Some(result) = memory.get(&(stone, count)) {
        *result
    } else {
        let result = if stone == 0 {
            memo(memory, 1, count - 1)
        } else {
            let num_digits = stone.ilog10() + 1;
            if num_digits % 2 == 0 {
                let left = stone / 10u64.pow(num_digits / 2);
                let right = stone % 10u64.pow(num_digits / 2);
                memo(memory, left, count - 1) + memo(memory, right, count - 1)
            } else {
                memo(memory, stone * 2024, count - 1)
            }
        };

        memory.insert((stone, count), result);
        result
    }
}

type Stones = Vec<u64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(include_str!("../input/2024/day11-example.txt")),
            55312
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input/2024/day11-example.txt")), 6);
    }
}
