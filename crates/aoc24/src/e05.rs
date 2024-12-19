use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, Lines, PuzzleResult};
use std::cmp::Ordering;
use std::collections::HashSet;

const DAY: Day = Day(5);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Ceres Search");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(input.lines()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 5991);

    let p2 = part2(input.lines()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 5479);

    Ok(())
}

type Page = i32;
type CompareSet = HashSet<(Page, Page)>;
type Update = Vec<Page>;

struct Input {
    ordering: CompareSet,
    updates: Vec<Update>,
}

fn part1(lines: Lines) -> PuzzleResult<i32> {
    let Input { ordering, updates } = parse(lines);

    let sum = updates
        .into_iter()
        .filter(|update| is_sorted(&ordering, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(sum)
}

fn part2(lines: Lines) -> PuzzleResult<i32> {
    let Input { ordering, updates } = parse(lines);

    let sum = updates
        .into_iter()
        .filter(|update| !is_sorted(&ordering, update))
        .map(|update| sorted(&ordering, &update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(sum)
}

fn parse(lines: Lines) -> Input {
    let lines: Vec<_> = lines.collect();
    let sections: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let ordering: CompareSet = sections[0]
        .iter()
        .map(|line| {
            let (first, second) = line.split_once('|').unwrap();
            let first: i32 = first.parse().unwrap();
            let second: i32 = second.parse().unwrap();
            (first, second)
        })
        .collect();

    let updates: Vec<_> = sections[1]
        .iter()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    Input { updates, ordering }
}

fn pair_compare(cmp: &CompareSet, a: &Page, b: &Page) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if cmp.contains(&(*a, *b)) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn is_sorted(ordering: &CompareSet, update: &Update) -> bool {
    update.is_sorted_by(|a, b| pair_compare(ordering, a, b) == Ordering::Less)
}

fn sorted(ordering: &CompareSet, update: &Update) -> Update {
    let mut pages = update.clone();
    pages.sort_by(|a, b| pair_compare(ordering, a, b));
    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE.into()).unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE.into()).unwrap(), 123);
    }
}
