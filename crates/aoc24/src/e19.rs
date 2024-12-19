use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleResult};
use fxhash::FxHashMap;
use regex::Regex;

const DAY: Day = Day(19);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Linen Layout");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 363);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 642535800868438);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    let linen = Linen::from(input.read_to_string()?.as_str());
    let regex = linen.build_regex();
    let count = linen
        .requested_patterns
        .iter()
        .filter(|pattern| regex.is_match(pattern))
        .count();

    Ok(count)
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    let linen = Linen::from(input.read_to_string()?.as_str());
    let mut memo = FxHashMap::default();
    let count = linen
        .requested_patterns
        .iter()
        .map(|pattern| match_pattern(pattern, &linen.towels, &mut memo))
        .sum::<usize>();
    Ok(count)
}

fn match_pattern(pattern: &str, towels: &[String], memo: &mut FxHashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(pattern) {
        return result;
    }

    let result = towels
        .iter()
        .filter_map(|towel| pattern.strip_prefix(towel))
        .map(|tail| match_pattern(tail, towels, memo))
        .sum();

    memo.insert(pattern.to_string(), result);
    result
}

struct Linen {
    towels: Vec<String>,
    requested_patterns: Vec<String>,
}

impl Linen {
    fn build_regex(&self) -> Regex {
        let regex = format!(r"^({})*$", self.towels.join("|"));
        Regex::new(&regex).expect("Invalid regex")
    }
}

impl From<&str> for Linen {
    fn from(input: &str) -> Self {
        let (towels, requested_patterns) = input.split_once("\n\n").expect("No empty line");
        Self {
            towels: towels.split(", ").map(str::to_string).collect(),
            requested_patterns: requested_patterns.lines().map(str::to_string).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 16);
    }
}
