use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};
use fancy_regex::Regex;
use std::io::{BufRead, BufReader, Read};

pub fn doesnt_he_have_intern_elves_for_this(
    day: u8,
    input: Box<dyn PuzzleInput>,
) -> PuzzleResult<bool> {
    header(day, "Doesn't He Have Intern-Elves For This?");
    let _input = input
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Failed to read the input for day {day}: {e}")))?;

    let nice_count = count_nice_ones(input.input()?)?;
    println!("aoc15e05a: {nice_count}");

    let really_nice_count = count_really_nice_ones(input.input()?)?;
    println!("aoc15e05b: {really_nice_count}");

    Ok(nice_count == 255 && really_nice_count == 55)
}

fn count_nice_ones(reader: BufReader<Box<dyn Read>>) -> PuzzleResult<usize> {
    let mut count = 0usize;
    for line in reader.lines() {
        let line = line.map_err(|e| {
            PuzzleError::Processing(format!("Failed to read a line: {e}"), e.into())
        })?;
        if has_three_vowels(&line)
            && has_duplicated_letters(&line)
            && !contains_forbidden_sequence(&line)
        {
            count += 1;
        }
    }

    Ok(count)
}

const REPEATED_PAIRS_REGEX: &str = r"(..).*\1";
const REPEATED_CHARS_REGEX: &str = r"(.).\1";

fn count_really_nice_ones(reader: BufReader<Box<dyn Read>>) -> PuzzleResult<usize> {
    let patterns = [
        Regex::new(REPEATED_CHARS_REGEX).unwrap(),
        Regex::new(REPEATED_PAIRS_REGEX).unwrap(),
    ];

    let count = reader
        .lines()
        .map(|line| {
            line.map_err(|e| PuzzleError::Processing(format!("Failed to read line: {e}"), e.into()))
        })
        .try_fold(0, |acc, line| {
            line.map(|l| {
                if patterns.iter().all(|p| matches(p, &l)) {
                    acc + 1
                } else {
                    acc
                }
            })
        })?;

    Ok(count)
}

fn has_three_vowels(s: impl AsRef<str>) -> bool {
    s.as_ref()
        .chars()
        .filter(|&c| "aeiou".contains(c.to_ascii_lowercase()))
        .count()
        >= 3
}

fn has_duplicated_letters(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
}

fn contains_forbidden_sequence(s: impl AsRef<str>) -> bool {
    ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|&seq| s.as_ref().contains(seq))
}

fn matches(pattern: &Regex, s: impl AsRef<str>) -> bool {
    pattern.is_match(s.as_ref()).unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_three_vowels() {
        assert!(!has_three_vowels(""));
        assert!(!has_three_vowels("xyz"));
        assert!(!has_three_vowels("a"));
        assert!(!has_three_vowels("aa"));
        assert!(has_three_vowels("aei"));
        assert!(has_three_vowels("iou"));
        assert!(has_three_vowels("xazegov"));
        assert!(has_three_vowels("aeiouaeiouaeiou"));
    }

    #[test]
    fn test_contains_consecutive_pairs() {
        let pattern = Regex::new(REPEATED_PAIRS_REGEX).unwrap();
        let contains_consecutive_pairs = |s| matches(&pattern, &s);

        assert!(contains_consecutive_pairs("xyxy"));
        assert!(contains_consecutive_pairs("aabcdefgaa"));
        assert!(!contains_consecutive_pairs("aaa"));
    }

    #[test]
    fn test_contains_interspersed_repeated_character() {
        let pattern = Regex::new(REPEATED_CHARS_REGEX).unwrap();
        let has_interspersed_repeated_char = |s| matches(&pattern, &s);

        assert!(has_interspersed_repeated_char("aba"));
        assert!(has_interspersed_repeated_char("aaa"));
        assert!(!has_interspersed_repeated_char("aa"));
        assert!(!has_interspersed_repeated_char("aab"));
    }
}