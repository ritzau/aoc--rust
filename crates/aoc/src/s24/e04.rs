use crate::cache::AocCache;
use crate::input::{InputFetcher, Lines};
use crate::s24::e04::DiagonalDirection::{DownRight, UpRight};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use std::collections::HashSet;
use std::iter::Chain;

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

fn part1(lines: Lines) -> PuzzleResult<usize> {
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let count = combined(&matrix)
        .map(|(s, _)| count_matches("XMAS", "SAMX", s))
        .sum();

    Ok(count)
}

fn part2(lines: Lines) -> PuzzleResult<usize> {
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let down_right_coords = diagonal_iterator(down_right_diagonals(&matrix));
    let up_right_coords = diagonal_iterator(up_right_diagonals(&matrix));

    Ok(down_right_coords.intersection(&up_right_coords).count())
}

fn diagonal_iterator(iterator: DiagonalIterator) -> HashSet<(usize, usize)> {
    const SEARCH: &str = "MAS";
    const REV_SEARCH: &str = "SAM";

    let mut coords: HashSet<(usize, usize)> = HashSet::new();

    for (s, map) in iterator {
        let indices = matching_indices(SEARCH, REV_SEARCH, &s);
        let mut indices = indices.into_iter().map(|i| map[i]);
        coords.extend(&mut indices);
    }

    coords
}

fn count_matches(search: &str, rev_search: &str, s: String) -> usize {
    s.matches(search).count() + s.matches(rev_search).count()
}

fn matching_indices(search: &str, rev_search: &str, s: &str) -> Vec<usize> {
    let mut indices = Vec::new();

    for i in 0..s.len() {
        if s[i..].starts_with(search) {
            indices.push(i + search.len() / 2);
        }
        if s[i..].starts_with(rev_search) {
            indices.push(i + rev_search.len() / 2);
        }
    }

    indices
}

struct RowsIterator<'a> {
    matrix: &'a [Vec<char>],
    row_index: usize,
}

impl<'a> Iterator for RowsIterator<'a> {
    type Item = (String, Vec<(usize, usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= self.matrix.len() {
            return None; // No more rows
        }

        let row = &self.matrix[self.row_index];
        let diagonal = row.iter().collect();
        let mapping: Vec<(usize, usize)> = (0..row.len()).map(|c| (self.row_index, c)).collect();

        self.row_index += 1;
        Some((diagonal, mapping))
    }
}

fn rows(matrix: &[Vec<char>]) -> RowsIterator {
    RowsIterator {
        matrix,
        row_index: 0,
    }
}

struct ColumnsIterator<'a> {
    matrix: &'a [Vec<char>],
    col_index: usize,
}

impl<'a> Iterator for ColumnsIterator<'a> {
    type Item = (String, Vec<(usize, usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col_index >= self.matrix[0].len() {
            return None; // No more columns
        }

        let column: String = self.matrix.iter().map(|row| row[self.col_index]).collect();
        let mapping: Vec<(usize, usize)> = (0..self.matrix.len())
            .map(|r| (r, self.col_index))
            .collect();

        self.col_index += 1;
        Some((column, mapping))
    }
}

fn columns(matrix: &[Vec<char>]) -> ColumnsIterator {
    ColumnsIterator {
        matrix,
        col_index: 0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DiagonalDirection {
    DownRight,
    UpRight,
}

struct DiagonalIterator<'a> {
    matrix: &'a [Vec<char>],
    offset: i32,
    n: i32,
    direction: DiagonalDirection,
}

impl<'a> Iterator for DiagonalIterator<'a> {
    type Item = (String, Vec<(usize, usize)>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.n {
            return None; // No more diagonals
        }

        let direction = match self.direction {
            UpRight => -1,
            DownRight => 1,
        };
        let right_down_start_row = if self.offset < 0 { -self.offset } else { 0 };
        let start_row = match self.direction {
            UpRight => self.n - 1 - right_down_start_row,
            DownRight => right_down_start_row,
        };
        let start_col = if self.offset < 0 { 0 } else { self.offset };
        let diagonal_length = self.n - right_down_start_row.max(start_col);

        let mut diagonal = String::new();
        let mut mapping = Vec::new();

        for i in 0..diagonal_length {
            let r = start_row + direction * i;
            let c = start_col + i;

            diagonal.push(self.matrix[r as usize][c as usize]);
            mapping.push((r as usize, c as usize));
        }

        self.offset += 1;
        Some((diagonal, mapping))
    }
}

fn down_right_diagonals(matrix: &[Vec<char>]) -> DiagonalIterator {
    DiagonalIterator {
        matrix,
        offset: -((matrix.len() as i32) - 1),
        n: matrix.len() as i32,
        direction: DownRight,
    }
}

fn up_right_diagonals(matrix: &[Vec<char>]) -> DiagonalIterator {
    DiagonalIterator {
        matrix,
        offset: -((matrix.len() as i32) - 1),
        n: matrix.len() as i32,
        direction: UpRight,
    }
}

fn combined(
    matrix: &[Vec<char>],
) -> Chain<Chain<Chain<RowsIterator, ColumnsIterator>, DiagonalIterator>, DiagonalIterator> {
    rows(matrix)
        .chain(columns(matrix))
        .chain(down_right_diagonals(matrix))
        .chain(up_right_diagonals(matrix))
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
