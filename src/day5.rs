use aoc_runner_derive::aoc;
use bitvec::bitvec;
use bitvec::vec::BitVec;

fn parse(input: &str) -> PageRules {
    let mut page_rules = PageRules::default();

    let mut lines = input.lines();

    page_rules.rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once("|").unwrap();
            let before = before.parse().unwrap();
            let after = after.parse().unwrap();
            page_rules.max_pages = page_rules.max_pages.max(before + 1).max(after + 1);
            Rule { before, after }
        })
        .collect();

    page_rules.updates = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|page| {
                    let page = page.parse().unwrap();
                    page_rules.max_pages = page_rules.max_pages.max(page + 1);
                    page
                })
                .collect()
        })
        .collect();

    page_rules
}

#[derive(Default)]
struct PageRules {
    max_pages: usize,
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

struct Rule {
    before: usize,
    after: usize,
}

type Update = Vec<usize>;

struct DependencyTable {
    max_pages: usize,
    table: BitVec,
}

impl DependencyTable {
    fn new(max_pages: usize) -> Self {
        let size = max_pages * max_pages;
        Self {
            max_pages,
            table: bitvec![1; size],
        }
    }

    fn set(&mut self, before: usize, after: usize, value: bool) {
        let index = after * self.max_pages + before;
        self.table.set(index, value);
    }

    fn get(&self, before: usize, after: usize) -> bool {
        let index = after * self.max_pages + before;
        self.table[index]
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let page_rules = parse(input);

    let mut total = 0;

    // A value of "false" means there's a dependency
    let mut dependency_table = DependencyTable::new(page_rules.max_pages);

    for rule in &page_rules.rules {
        dependency_table.set(rule.before, rule.after, false);
    }

    // A value of "true" means it satisfies that dependency
    let mut update_table = DependencyTable::new(page_rules.max_pages);

    for update in &page_rules.updates {
        update_table.table.fill(true);

        for i in 0..update.len() {
            let before = update[i];
            for j in (i + 1)..update.len() {
                let after = update[j];
                update_table.set(after, before, false);
            }
        }

        // Satisfied for each pair of page numbers A, B iff
        // "A is before B" OR "there's no rule that A is before B"
        update_table.table |= &dependency_table.table;

        if update_table.table.all() {
            let middle = update[update.len() / 2];
            total += middle;
        }
    }

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut page_rules = parse(input);

    let mut total = 0;

    // A value of "false" means there's a dependency
    let mut dependency_table = DependencyTable::new(page_rules.max_pages);

    for rule in &page_rules.rules {
        dependency_table.set(rule.before, rule.after, false);
    }

    // A value of "true" means it satisfies that dependency
    let mut update_table = DependencyTable::new(page_rules.max_pages);

    for update in &mut page_rules.updates {
        update_table.table.fill(true);

        for i in 0..update.len() {
            let before = update[i];
            for j in (i + 1)..update.len() {
                let after = update[j];
                update_table.set(after, before, false);
            }
        }

        // Satisfied for each pair of page numbers A, B iff
        // "A is before B" OR "there's no rule that A is before B"
        update_table.table |= &dependency_table.table;

        if !update_table.table.all() {
            let sorted = sort_pages(update, &dependency_table);
            let middle = sorted[sorted.len() / 2];
            total += middle;
        }
    }

    total
}

fn sort_pages(pages: &[usize], dependency_table: &DependencyTable) -> Vec<usize> {
    let mut no_dependencies = Vec::<usize>::new();
    let mut dependency_counts = vec![0; dependency_table.max_pages];

    let mut pages_absent = bitvec![1; dependency_table.max_pages];
    for page in pages.iter() {
        pages_absent.set(*page, false);
    }

    let mut dependencies = bitvec![0; dependency_table.max_pages];

    for page in pages.iter() {
        dependencies.copy_from_bitslice(
            &dependency_table.table
                [*page * dependency_table.max_pages..(*page + 1) * dependency_table.max_pages],
        );

        dependencies |= &pages_absent;

        let dependency_count = dependencies.count_zeros();
        dependency_counts[*page] = dependency_count;

        if dependency_count == 0 {
            no_dependencies.push(*page);
        }
    }

    let mut sorted = Vec::with_capacity(pages.len());

    while let Some(page) = no_dependencies.pop() {
        sorted.push(page);

        for dependency in pages.iter() {
            if !dependency_table.get(page, *dependency) {
                dependency_counts[*dependency] -= 1;
                if dependency_counts[*dependency] == 0 {
                    no_dependencies.push(*dependency);
                }
            }
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input/2024/day5-example.txt")), 123);
    }

    #[test]
    fn test_part2_minimal() {
        part2(include_str!("../input/2024/day5-minimal.txt"));
    }
}
