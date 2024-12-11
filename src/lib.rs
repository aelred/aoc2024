#[macro_use]
extern crate aoc_runner_derive;

mod bitset;

// Adding #[macro_use] prevents rustfmt re-ordering them
// (#[rustfmt::skip] seems to break the aoc runner for some reason...)
#[macro_use]
pub mod day1;
pub mod day2;
#[macro_use]
pub mod day3;
#[macro_use]
pub mod day4;
#[macro_use]
pub mod day5;
#[macro_use]
pub mod day6;
#[macro_use]
pub mod day11;

aoc_lib! { year = 2024 }
