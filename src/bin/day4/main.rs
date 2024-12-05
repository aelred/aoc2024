use std::fmt::Display;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let wordsearch = parse();

    println!("Part 1: {}", part1(&wordsearch));

    println!("Part 2: {}", part2(&wordsearch));

    println!("Time: {}μs", start.elapsed().as_micros());
}

fn parse() -> Vec<Vec<char>> {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

fn part1(wordsearch: &[Vec<char>]) -> impl Display {
    let mut matches = 0;

    for y in 0..wordsearch.len() {
        let row = &wordsearch[y];
        for x in 0..row.len() {
            let char = row[x];
            if char != WORD[0] {
                continue;
            }
            for direction in DIRECTIONS {
                if has_xmas(wordsearch, x, y, direction) {
                    matches += 1;
                }
            }
        }
    }

    matches
}

fn has_xmas(wordsearch: &[Vec<char>], x: usize, y: usize, [dx, dy]: [isize; 2]) -> bool {
    let mut new_x = x as isize;
    let mut new_y = y as isize;
    for expected in &WORD[1..WORD.len()] {
        new_x += dx;
        new_y += dy;
        let actual = get_cell(wordsearch, new_x as usize, new_y as usize);
        if actual != Some(*expected) {
            return false;
        }
    }

    true
}

fn part2(wordsearch: &[Vec<char>]) -> impl Display {
    let mut matches = 0;

    for y in 0..wordsearch.len() {
        let row = &wordsearch[y];
        for x in 0..row.len() {
            if has_x_mas(wordsearch, x, y) {
                matches += 1;
            }
        }
    }

    matches
}

fn has_x_mas(wordsearch: &[Vec<char>], x: usize, y: usize) -> bool {
    let cell = wordsearch[y][x];
    let tl = get_cell(wordsearch, x - 1, y - 1);
    let tr = get_cell(wordsearch, x + 1, y - 1);
    let bl = get_cell(wordsearch, x - 1, y + 1);
    let br = get_cell(wordsearch, x + 1, y + 1);

    fn is_ms(cell: Option<char>) -> bool {
        cell == Some('M') || cell == Some('S')
    }

    cell == 'A' && is_ms(tl) && is_ms(tr) && is_ms(bl) && is_ms(br) && tl != br && tr != bl
}

fn get_cell(wordsearch: &[Vec<char>], x: usize, y: usize) -> Option<char> {
    wordsearch.get(y).and_then(|row| row.get(x)).copied()
}
