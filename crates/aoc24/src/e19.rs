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

    let encoded_towels: Vec<_> = linen
        .towels
        .iter()
        .map(|p| (encode_string(p).0, p.len()))
        .collect();

    let count = linen
        .requested_patterns
        .iter()
        .map(|p| (encode_string(p), p.len()))
        .map(|(pattern, pattern_len)| {
            match_pattern(pattern, pattern_len, &encoded_towels, &mut memo)
        })
        .sum::<usize>();
    Ok(count)
}

const WHITE: u8 = 0;
const BLUE: u8 = 1;
const BLACK: u8 = 2;
const RED: u8 = 3;
const GREEN: u8 = 4;

fn encode_color(c: char) -> u8 {
    match c {
        'w' => WHITE,
        'u' => BLUE,
        'b' => BLACK,
        'r' => RED,
        'g' => GREEN,
        _ => panic!("Invalid color"),
    }
}

// Store 42 characters in each element of the tuple
type EncodedString = (u128, u128);

// Encode each character in a string into 3 bits
fn encode_string(s: &str) -> EncodedString {
    let mut encoded = (0u128, 0u128);
    let mut shift = 0;

    for c in s.chars() {
        if shift >= 126 {
            encoded.1 |= (encode_color(c) as u128) << (shift - 126);
        } else {
            encoded.0 |= (encode_color(c) as u128) << shift;
        }
        shift += 3;
    }

    encoded
}

fn match_pattern(
    pattern: EncodedString,
    pattern_len: usize,
    towels: &[(u128, usize)],
    memo: &mut FxHashMap<(EncodedString, usize), usize>,
) -> usize {
    if pattern_len == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&(pattern, pattern_len)) {
        return result;
    }

    let mut result = 0;
    for &(towel, towel_len) in towels.iter().filter(|&&(_, len)| len <= pattern_len) {
        let mask = towel_mask(towel_len);

        if is_prefix(towel, pattern.0, mask) {
            let tail = pattern_tail(pattern, towel_len);
            result += match_pattern(tail, pattern_len - towel_len, towels, memo);
        }
    }

    memo.insert((pattern, pattern_len), result);
    result
}

// All towels fit in a single u128
fn is_prefix(towel: u128, pattern: u128, mask: u128) -> bool {
    (towel & mask) == (pattern & mask)
}

fn towel_mask(len: usize) -> u128 {
    (1 << (len * 3)) - 1
}

fn pattern_tail(pattern: EncodedString, len: usize) -> (u128, u128) {
    let shift = len * 3;
    if shift > 126 {
        (pattern.1 >> (shift - 126), 0)
    } else {
        (
            (pattern.0 >> shift) | (pattern.1 << (126 - shift)),
            pattern.1 >> shift,
        )
    }
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

    #[test]
    fn test_pattern_mask() {
        // Test case where pattern_len * 3 <= 126
        assert_eq!(towel_mask(10), (1 << 30) - 1);
    }

    #[test]
    fn test_pattern_tail() {
        // Test case where pattern_len * 3 <= 126
        let pattern = (0b1111, 0);
        let towel_len = 1;
        let expected = (0b1, 0);
        assert_eq!(pattern_tail(pattern, towel_len), expected);

        // Test case where pattern_len * 3 <= 126
        let pattern = (0b111111111111111111111111111111, 0);
        let towel_len = 10;
        let expected = (0, 0);
        assert_eq!(pattern_tail(pattern, towel_len), expected);

        // Test case where pattern_len * 3 > 126
        let pattern = (u128::MAX, (1 << 25) - 1);
        let towel_len = 50;
        let expected = (1, 0);
        assert_eq!(pattern_tail(pattern, towel_len), expected);

        // Additional test case
        let pattern = (
            0b111111111111111111111111111111,
            0b111111111111111111111111111111,
        );
        let towel_len = 42;
        let expected = (0b111111111111111111111111111111, 0);
        assert_eq!(pattern_tail(pattern, towel_len), expected);
    }
}
