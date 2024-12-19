use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleResult};
use rayon::prelude::*;

const DAY: Day = Day(19);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Linen Layout");
    let input = aoc.get_input(YEAR, DAY)?;

    let (p1, p2) = part_1_and_2(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 363);

    println!("Part 2: {}", p2);
    assert_eq!(p2, 642535800868438);

    Ok(())
}

fn part_1_and_2(input: &Input) -> PuzzleResult<(usize, usize)> {
    let linen = Linen::from(input.read_to_string()?.as_str());

    let mut towels: Vec<Vec<(u128, usize)>> = vec![Vec::new(); 8];
    linen
        .towels
        .iter()
        .map(|p| (encode_string(p).0, p.len()))
        .for_each(|(towel, len)| {
            let towel_ind = towel_index(towel);
            towels[towel_ind].push((towel, len));
        });

    let (count, sum) = linen
        .requested_patterns
        .par_iter()
        .map(|p| (encode_string(p), p.len()))
        .map(|(pattern, pattern_len)| {
            let result = match_pattern(pattern, pattern_len, &towels, &mut [usize::MAX; 84]);
            ((result > 0) as usize, result)
        })
        .reduce(
            || (0, 0),
            |(count1, sum1), (count2, sum2)| (count1 + count2, sum1 + sum2),
        );

    Ok((count, sum))
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
    towels: &[Vec<(u128, usize)>],
    memo: &mut [usize; 84],
) -> usize {
    if pattern_len == 0 {
        return 1;
    }

    if memo[pattern_len] != usize::MAX {
        return memo[pattern_len];
    }

    let towel_ind = towel_index(pattern.0);
    let result: usize = towels[towel_ind]
        .iter()
        .filter(|&&(_, len)| len <= pattern_len)
        .filter_map(|&(towel, towel_len)| {
            let mask = towel_mask(towel_len);
            if is_prefix(towel, pattern.0, mask) {
                let tail = pattern_tail(pattern, towel_len);
                Some(match_pattern(tail, pattern_len - towel_len, towels, memo))
            } else {
                None
            }
        })
        .sum();

    memo[pattern_len] = result;
    result
}

fn towel_index(towel: u128) -> usize {
    (towel & 0b111) as usize
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
    fn test_parts() {
        let (p1, p2) = part_1_and_2(&SAMPLE.into()).unwrap();
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
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
