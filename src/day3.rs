use aoc_runner_derive::aoc;
use regex::Regex;
use std::str::FromStr;

fn parse(input: &str) -> Vec<Instr> {
    let mut instrs = Vec::new();
    let mul = Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    for result in mul.captures_iter(input) {
        let is_mul = result.get(1).is_some_and(|s| !s.is_empty());
        let is_do = result.get(4).is_some_and(|s| !s.is_empty());
        let is_dont = result.get(5).is_some_and(|s| !s.is_empty());

        let instr = if is_mul {
            let x = u64::from_str(result.get(2).unwrap().as_str()).unwrap();
            let y = u64::from_str(result.get(3).unwrap().as_str()).unwrap();
            Instr::Mul { x, y }
        } else if is_do {
            Instr::Do
        } else if is_dont {
            Instr::Dont
        } else {
            panic!("Unreachable")
        };

        instrs.push(instr);
    }

    instrs
}

enum Instr {
    Mul { x: u64, y: u64 },
    Do,
    Dont,
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let instructions = parse(input);

    let mut total = 0;

    for instr in instructions {
        if let Instr::Mul { x, y } = instr {
            total += x * y;
        }
    }

    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let instructions = parse(input);

    let mut total = 0;
    let mut enable_muls = true;

    for instr in instructions {
        match instr {
            Instr::Mul { x, y } => {
                if enable_muls {
                    total += x * y;
                }
            }
            Instr::Do => enable_muls = true,
            Instr::Dont => enable_muls = false,
        }
    }

    total
}
