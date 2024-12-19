use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleError, PuzzleResult};
use itertools::Itertools;

const DAY: Day = Day(10);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Hoof It");
    let input = aoc.get_input(YEAR, DAY)?.read_to_string()?;
    let (p2, p1) = parts2and1(&input)?;

    println!("Part 1: {}", p1);
    assert_eq!(p1, 644);

    println!("Part 2: {}", p2);
    assert_eq!(p2, 1366);

    Ok(())
}

fn parts2and1(input: &str) -> PuzzleResult<(usize, usize)> {
    let mut grid = [[-1; 64]; 64];
    parse(input, &mut grid)?;
    let scores = find_heads(&grid)
        .into_iter()
        .map(|h| find_peak(h, &grid))
        .fold((0, 0), |(acc1, acc2), (val1, val2)| {
            (acc1 + val1, acc2 + val2)
        });

    Ok(scores)
}

fn parse<const N: usize>(input: &str, grid: &mut [[i8; N]; N]) -> PuzzleResult<()> {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[1 + i][1 + j] = c
                .to_digit(10)
                .unwrap()
                .try_into()
                .map_err(|_| PuzzleError::Input(format!("Not a digit: {c}")))?;
        }
    }

    Ok(())
}

fn find_heads<const N: usize>(grid: &[[i8; N]; N]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, h)| **h == 0)
                .map(move |(c, _)| (r, c))
        })
        .collect()
}

fn find_peak<const N: usize>(head: (usize, usize), grid: &[[i8; N]; N]) -> (usize, usize) {
    let mut peaks = Vec::new();
    let mut queue = vec![head];
    while let Some(pos) = queue.pop() {
        let height = grid[pos.0][pos.1];
        if height == 9 {
            peaks.push(pos);
            continue;
        }

        let moves = [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        moves
            .into_iter()
            .filter(|m| grid[m.0][m.1] == height + 1)
            .for_each(|m| queue.push(m));
    }

    (peaks.len(), peaks.into_iter().unique().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        let (_, p1) = parts2and1(SAMPLE).unwrap();
        assert_eq!(p1, 36);
    }

    #[test]
    fn test_part2() {
        let (p2, _) = parts2and1(SAMPLE).unwrap();
        assert_eq!(p2, 81);
    }
}
