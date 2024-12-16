use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use fxhash::FxHashMap;
use std::collections::VecDeque;

const DAY: Day = Day(12);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Garden Groups");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 1477924);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 841934);

    Ok(())
}

const N: usize = 140;

#[derive(Clone, Debug)]
struct Map {
    grid: [[(char, u16); N]; N],
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    None,
    A,
    B,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut grid = [[(' ', 0); N]; N];
        let mut width = 0;
        let mut height = 0;

        input.lines().enumerate().for_each(|(r, line)| {
            width = width.max(line.len());
            height = height.max(r + 1);
            line.chars().enumerate().for_each(|(c, ch)| {
                grid[r][c].0 = ch;
            });
        });

        Self {
            grid,
            width,
            height,
        }
    }

    fn identify_regions(&mut self) -> FxHashMap<u16, (u16, u16)> {
        let mut next_id = 1u16;
        let mut regions = FxHashMap::<u16, (u16, u16)>::default();

        for r in 0..self.height {
            for c in 0..self.width {
                if self.grid[r][c].1 == 0 {
                    let id = next_id;
                    next_id += 1;

                    let ch = self.grid[r][c].0;
                    let mut area = 0u16;
                    let mut perimeter = 0u16;
                    let mut queue = VecDeque::<(usize, usize)>::new();
                    self.grid[r][c].1 = id;
                    queue.push_back((r, c));

                    while let Some((r, c)) = queue.pop_back() {
                        assert_ne!(self.grid[r][c].1, 0);

                        area += 1;
                        perimeter += 4;

                        if r > 0 && self.grid[r - 1][c].0 == ch {
                            perimeter -= 1;
                            if self.grid[r - 1][c].1 == 0 {
                                self.grid[r - 1][c].1 = id;
                                queue.push_back((r - 1, c));
                            }
                        }
                        if r < self.height - 1 && self.grid[r + 1][c].0 == ch {
                            perimeter -= 1;
                            if self.grid[r + 1][c].1 == 0 {
                                self.grid[r + 1][c].1 = id;
                                queue.push_back((r + 1, c));
                            }
                        }
                        if c > 0 && self.grid[r][c - 1].0 == ch {
                            perimeter -= 1;
                            if self.grid[r][c - 1].1 == 0 {
                                self.grid[r][c - 1].1 = id;
                                queue.push_back((r, c - 1));
                            }
                        }
                        if c < self.width - 1 && self.grid[r][c + 1].0 == ch {
                            perimeter -= 1;
                            if self.grid[r][c + 1].1 == 0 {
                                self.grid[r][c + 1].1 = id;
                                queue.push_back((r, c + 1));
                            }
                        }
                    }

                    regions.insert(id, (area, perimeter));
                }
            }
        }

        regions
    }

    fn count_sides(&self, id: u16) -> usize {
        let handle_edge = |cell: u16, side: &mut Side| {
            if cell == id {
                if *side != Side::A {
                    *side = Side::A;
                    return 1;
                }
            } else {
                *side = Side::None;
            }
            0
        };

        let handle_mid = |upper: u16, lower: u16, side: &mut Side| {
            if upper == id && lower != id {
                if *side != Side::B {
                    *side = Side::B;
                    return 1;
                }
            } else if upper != id && lower == id {
                if *side != Side::A {
                    *side = Side::A;
                    return 1;
                }
            } else {
                *side = Side::None;
            }
            0
        };

        let mut count = 0usize;

        // Top line
        let mut side = Side::None;
        for c in 0..self.width {
            let cell = self.grid[0][c].1;
            count += handle_edge(cell, &mut side);
        }

        // Bottom line
        let mut side = Side::None;
        for c in 0..self.width {
            let cell = self.grid[self.height - 1][c].1;
            count += handle_edge(cell, &mut side);
        }

        // Left side
        let mut side = Side::None;
        for r in 0..self.height {
            let cell = self.grid[r][0].1;
            count += handle_edge(cell, &mut side);
        }

        // Right side
        let mut side = Side::None;
        for r in 0..self.height {
            let cell = self.grid[r][self.width - 1].1;
            count += handle_edge(cell, &mut side);
        }

        // Mid-lines
        for r in 0..self.height - 1 {
            let mut side = Side::None;
            for c in 0..self.width {
                let upper = self.grid[r][c].1;
                let lower = self.grid[r + 1][c].1;
                count += handle_mid(upper, lower, &mut side);
            }
        }

        // Mid-cols
        for c in 0..self.width - 1 {
            let mut side = Side::None;
            for r in 0..self.height {
                let left = self.grid[r][c].1;
                let right = self.grid[r][c + 1].1;
                count += handle_mid(left, right, &mut side);
            }
        }

        count
    }
}

fn part1(input: &Input) -> PuzzleResult<i32> {
    let mut map = Map::parse(&input.read_to_string()?);
    let regions = map.identify_regions();
    let cost = regions
        .values()
        .map(|&(area, perimeter)| area as i32 * perimeter as i32)
        .sum();
    Ok(cost)
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    let mut map = Map::parse(&input.read_to_string()?);
    let cost = map
        .identify_regions()
        .iter()
        .map(|(&id, &(area, _))| map.count_sides(id) * area as usize)
        .sum();

    Ok(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 1206);
    }
}
