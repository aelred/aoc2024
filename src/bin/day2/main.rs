use std::cmp::Ordering;
use std::str::FromStr;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let reports = parse();

    let safe_reports = part1(&reports);
    println!("Part 1: {}", safe_reports);

    let safe_dampened_reports = part2(&reports);
    println!("Part 2: {}", safe_dampened_reports);

    println!("Time: {}μs", start.elapsed().as_micros());
}

fn parse() -> Vec<Report> {
    let mut reports = Vec::new();

    for line in INPUT.lines() {
        let mut report = Report::default();

        for level in line.split_whitespace().collect::<Vec<_>>() {
            report.levels.push(u64::from_str(level).unwrap());
        }

        reports.push(report);
    }

    reports
}

#[derive(Default, Clone)]
struct Report {
    levels: Vec<u64>,
}

fn part1(reports: &[Report]) -> usize {
    reports.into_iter().filter(|r| is_safe(r)).count()
}

fn is_safe(report: &Report) -> bool {
    let increasing_or_decreasing = report.levels[0].cmp(report.levels.last().unwrap());

    if increasing_or_decreasing == Ordering::Equal {
        return false;
    }

    for window in report.levels.windows(2) {
        if window[0].cmp(&window[1]) != increasing_or_decreasing {
            return false;
        }

        if window[0].abs_diff(window[1]) > 3 {
            return false;
        }
    }

    true
}

fn part2(reports: &[Report]) -> usize {
    reports.into_iter().filter(|r| is_safe_dampened(r)).count()
}

fn is_safe_dampened(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.levels.len() {
        let mut reduced_report = report.clone();
        reduced_report.levels.remove(i);
        if is_safe(&reduced_report) {
            return true;
        }
    }

    false
}
