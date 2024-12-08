use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
use fxhash::FxHashMap;
use std::fmt;

const DAY: Day = Day(8);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Resonant Collinearity");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 301);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 1019);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    let input = input.read_to_string()?;
    let mut map = Map::from(&input)?;
    map.locate_first_antinodes();
    Ok(map.count_antinodes())
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    let input = input.read_to_string()?;
    let mut map = Map::from(&input)?;
    map.locate_all_antinodes();
    Ok(map.count_antinodes())
}

const MAX_SIDE: usize = 100;

#[derive(Debug)]
struct Map {
    grid: [[u8; MAX_SIDE]; MAX_SIDE],
    side: usize,
    antennas: FxHashMap<u8, Vec<(i32, i32)>>,
}

impl Map {
    fn from(input: &str) -> PuzzleResult<Self> {
        let lines: Vec<_> = input.lines().collect();

        let mut map = Self {
            grid: [[0; MAX_SIDE]; MAX_SIDE],
            side: lines.len(),
            antennas: FxHashMap::default(),
        };

        if !lines.iter().all(|line| line.len() == map.side) {
            return Err(PuzzleError::Input("Input is not square".to_string()));
        }

        for (line_no, line) in lines.into_iter().enumerate() {
            for (col_no, c) in line.bytes().enumerate() {
                if c != b'.' {
                    map.antennas
                        .entry(c)
                        .or_default()
                        .push((line_no as i32, col_no as i32));
                }
                map.grid[line_no][col_no] = c;
            }
        }

        Ok(map)
    }

    fn locate_first_antinodes(&mut self) {
        let side = self.side as i32;
        for (_, coords) in &self.antennas {
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    let (xi, yi) = coords[i];
                    let (xj, yj) = coords[j];
                    let (dx, dy) = (xi - xj, yi - yj);

                    let (xi_anti, yi_anti) = (xi + dx, yi + dy);
                    if 0 <= xi_anti && xi_anti < side && 0 <= yi_anti && yi_anti < side {
                        self.grid[xi_anti as usize][yi_anti as usize] = b'#';
                    }

                    let (xj_anti, yj_anti) = (xj - dx, yj - dy);
                    if 0 <= xj_anti && xj_anti < side && 0 <= yj_anti && yj_anti < side {
                        self.grid[xj_anti as usize][yj_anti as usize] = b'#';
                    }
                }
            }
        }
    }

    fn locate_all_antinodes(&mut self) {
        let side = self.side as i32;

        for (_, coords) in &self.antennas {
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    let (xi, yi) = coords[i];
                    let (xj, yj) = coords[j];
                    let (dx, dy) = (xi - xj, yi - yj);

                    let (mut px, mut py) = (xi, yi);
                    while 0 <= px && px < side && 0 <= py && py < side {
                        self.grid[px as usize][py as usize] = b'#';
                        px += dx;
                        py += dy;
                    }

                    let (mut px, mut py) = (xi, yi);
                    while 0 <= px && px < side && 0 <= py && py < side {
                        self.grid[px as usize][py as usize] = b'#';
                        px -= dx;
                        py -= dy;
                    }
                }
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        self.grid
            .into_iter()
            .take(self.side)
            .flat_map(|line| line.into_iter().take(self.side).filter(|&b| b == b'#'))
            .count()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (antenna, coords) in &self.antennas {
            writeln!(f, "Antenna {:?} at {:?}", *antenna as char, coords)?;
        }

        let output = self
            .grid
            .iter()
            .take(self.side)
            .map(|row| {
                row.iter()
                    .take(self.side)
                    .map(|&c| c as char)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 34);
    }
}
