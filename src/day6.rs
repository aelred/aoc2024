use crate::bitset::{BitSet, BitSetMember};
use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut map = Map::new(input);
    let mut positions = BitSet::default();

    loop {
        positions.add(map.guard_pos);
        if !map.move_guard() {
            break;
        }
    }

    positions.count()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut map = Map::new(input);

    let mut valid_positions = BitSet::default();

    loop {
        if !map.move_guard() {
            break;
        }
        valid_positions.add(map.guard_pos);
    }

    let mut loops = 0;

    for obstruction in valid_positions.iter() {
        map.guard_pos = map.initial_guard_pos;
        map.guard_dir = Vec2 { x: 0, y: -1 };

        let mut dir_positions = [
            BitSet::default(),
            BitSet::default(),
            BitSet::default(),
            BitSet::default(),
        ];

        map.obstructions.add(obstruction);

        let looped = loop {
            if dir_positions[map.guard_dir.to_dir_index()].add(map.guard_pos) {
                // loop detected!
                break true;
            }

            if !map.move_guard() {
                break false;
            }
        };

        map.obstructions.remove(obstruction);

        if looped {
            loops += 1;
        }
    }

    loops
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn to_dir_index(self) -> usize {
        (!((self.x + 1) | (self.y + 1)) & 0b10 | ((self.x + 1) & 0b1)) as usize
    }
}

impl BitSetMember for Vec2 {
    fn from_index(index: usize) -> Self {
        let x = (index / 200) as isize;
        let y = (index % 200) as isize;
        Self { x, y }
    }

    fn to_index(self) -> usize {
        debug_assert!(self.y < 200);
        self.x as usize * 200 + self.y as usize
    }

    fn max() -> Self {
        Self { x: 199, y: 199 }
    }
}

struct Map {
    obstructions: BitSet<Vec2>,
    initial_guard_pos: Vec2,
    guard_pos: Vec2,
    guard_dir: Vec2,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut obstructions = BitSet::default();
        let mut guard_pos = Vec2 { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.as_bytes().iter().enumerate() {
                if *char == b'#' {
                    obstructions.add(Vec2 {
                        x: x as isize,
                        y: y as isize,
                    });
                } else if *char == b'^' {
                    guard_pos = Vec2 {
                        x: x as isize,
                        y: y as isize,
                    }
                }
            }
        }

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Self {
            obstructions,
            initial_guard_pos: guard_pos,
            guard_pos,
            guard_dir: Vec2 { x: 0, y: -1 },
            width,
            height,
        }
    }

    fn move_guard(&mut self) -> bool {
        let next_pos = Vec2 {
            x: self.guard_pos.x + self.guard_dir.x,
            y: self.guard_pos.y + self.guard_dir.y,
        };

        if next_pos.x < 0
            || next_pos.x as usize >= self.width
            || next_pos.y < 0
            || next_pos.y as usize >= self.height
        {
            return false;
        } else if self.obstructions.has(next_pos) {
            // turn right
            self.guard_dir = Vec2 {
                x: -self.guard_dir.y,
                y: self.guard_dir.x,
            };
        } else {
            self.guard_pos = next_pos;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input/2024/day6-example.txt")), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input/2024/day6-example.txt")), 6);
    }
}
