use crate::cache::AocCache;
use crate::input::{InputFetcher, Lines};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use std::collections::HashSet;

const DAY: Day = Day(4);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Ceres Search");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(input.lines()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 2521);

    let p2 = part2(input.lines()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 1912);

    Ok(())
}

fn part1(lines: Lines) -> PuzzleResult<i32> {
    const SEARCH: &str = "XMAS";
    let rev_search: String = SEARCH.chars().rev().collect();

    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut count = 0;

    let n = matrix[0].len() as i32;

    for row in &matrix {
        let s: String = row.iter().collect();
        count += count_matches(SEARCH, &rev_search, s);
    }

    for col in 0..n {
        let s: String = matrix.iter().map(|row| row[col as usize]).collect();
        count += count_matches(SEARCH, &rev_search, s);
    }

    for col in -n..n {
        let s: String = (col..n)
            .filter(|c| (0..n).contains(&c) && (0..n).contains(&(c - col)))
            .map(|c| matrix[(c - col) as usize][c as usize])
            .collect();
        count += count_matches(SEARCH, &rev_search, s);

        let s: String = (col..n)
            .filter(|c| (0..n).contains(&c) && (0..n).contains(&(c - col)))
            .map(|c| matrix[(n - (c - col) - 1) as usize][c as usize])
            .collect();
        count += count_matches(SEARCH, &rev_search, s);
    }

    Ok(count)
}

fn part2(lines: Lines) -> PuzzleResult<i32> {
    const SEARCH: &str = "MAS";
    let rev_search: String = SEARCH.chars().rev().collect();

    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let n = matrix[0].len() as i32;
    let mut down_right_indices: HashSet<(i32, i32)> = HashSet::new();
    let mut up_right_indices: HashSet<(i32, i32)> = HashSet::new();

    for col in -n..n {
        let start = col.max(0);
        let end = n.min(n + col);

        if end - start < SEARCH.len() as i32 {
            continue;
        }

        let s: String = (start..end)
            .map(|c| matrix[(c - col) as usize][c as usize])
            .collect();

        let indices = matching_indices(SEARCH, &rev_search, &s);
        let mut indices = indices
            .into_iter()
            .map(|i| start + i as i32)
            .map(|i| (i - col, i));
        down_right_indices.extend(&mut indices);

        let s: String = (start..end)
            .map(|c| matrix[(n - 1 - (c - col)) as usize][c as usize])
            .collect();
        let indices = matching_indices(SEARCH, &rev_search, &s);

        let mut indices = indices
            .into_iter()
            .map(|i| start + i as i32)
            .map(|i| (n - 1 - (i - col), i));
        up_right_indices.extend(&mut indices);
    }

    Ok(down_right_indices.intersection(&up_right_indices).count() as i32)
}

fn count_matches(search: &str, rev_search: &str, s: String) -> i32 {
    s.matches(search).count() as i32 + s.matches(rev_search).count() as i32
}

fn matching_indices(search: &str, rev_search: &str, s: &str) -> Vec<usize> {
    let mut indices = Vec::new();

    for i in 0..s.len() {
        if s[i..].starts_with(search) {
            indices.push(i + 1);
        }
        if s[i..].starts_with(rev_search) {
            indices.push(i + 1);
        }
    }

    indices
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE.into()).unwrap(), 18)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE.into()).unwrap(), 9)
    }
}
