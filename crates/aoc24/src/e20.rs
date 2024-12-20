use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleResult};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

const DAY: Day = Day(20);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Race Condition");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 1289);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 982425);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    count_shortcuts(input, 2, 100)
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    count_shortcuts(input, 20, 100)
}

fn count_shortcuts(input: &Input, shortcut_length: usize, threshold: usize) -> PuzzleResult<usize> {
    let mut maze: Maze = Maze::from(input.read_to_string()?.as_str());
    maze.walk();
    let cheats = maze.find_cheats(shortcut_length, threshold);
    Ok(cheats)
}

type Tile = i16;
const GRID_SIZE: usize = 142;
const WALL_TILE: Tile = Tile::MAX;
const EMPTY_TILE: Tile = Tile::MAX - 1;

struct Maze {
    grid: [[Tile; GRID_SIZE]; GRID_SIZE],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn walk(&mut self) {
        let mut queue = VecDeque::from([self.start]);
        self.grid[self.start.1][self.start.0] = 0;

        while let Some((x, y)) = queue.pop_front() {
            let next = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (nx, ny) in next {
                match self.grid[ny][nx] {
                    WALL_TILE => continue,
                    EMPTY_TILE => {
                        self.grid[ny][nx] = self.grid[y][x] + 1;
                        queue.push_back((nx, ny));
                    }
                    _ => {}
                }
            }
        }
    }

    fn find_cheats(&self, len: usize, threshold: usize) -> usize {
        (0..self.height)
            .into_par_iter()
            .map(|y| self.count_shortcuts_for_line(y, len, threshold))
            .sum()
    }

    fn count_shortcuts_for_line(&self, y: usize, len: usize, threshold: usize) -> usize {
        (0..self.width)
            .map(|x| self.count_shortcuts_for_pos(x, y, len, threshold))
            .sum::<usize>()
    }

    fn count_shortcuts_for_pos(&self, x: usize, y: usize, len: usize, threshold: usize) -> usize {
        let len = len as isize;
        let last_y = self.height as isize - 1;
        let last_x = self.width as isize - 1;

        let tile = self.grid[y][x];
        if tile == WALL_TILE {
            return 0;
        }

        let x = x as isize;
        let y = y as isize;
        let threshold = threshold as isize;

        let mut count = 0;

        for ny in 0.max(-len + y)..=last_y.min(len + y) {
            let max_dx = len - (y - ny).abs();
            let first = 0.max(-max_dx + x);
            let last = last_x.min(max_dx + x);

            for nx in first..=last {
                let other_tile = self.grid[ny as usize][nx as usize];
                if other_tile == WALL_TILE {
                    continue;
                }

                let tile_distance = (other_tile - tile) as isize;
                let cheat_length = (nx - x).abs() + (ny - y).abs();
                let distance = tile_distance - cheat_length;

                if distance >= threshold {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.grid[y][x] {
                    WALL_TILE => '#',
                    EMPTY_TILE if (x, y) == self.start => 'S',
                    EMPTY_TILE if (x, y) == self.end => 'E',
                    EMPTY_TILE => '.',
                    _ => ((self.grid[y][x] % 10) as u8 + b'0') as char,
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let mut grid = [[EMPTY_TILE; GRID_SIZE]; GRID_SIZE];
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = match c {
                    '#' => WALL_TILE,
                    '.' => EMPTY_TILE,
                    'S' => {
                        assert_eq!(start, None, "Multiple start positions");
                        start = Some((x, y));
                        EMPTY_TILE
                    }
                    'E' => {
                        assert_eq!(end, None, "Multiple end positions");
                        end = Some((x, y));
                        EMPTY_TILE
                    }
                    _ => panic!("Invalid character in maze: {}", c),
                };
            }
        }

        Self {
            grid,
            start: start.expect("No start position"),
            end: end.expect("No end position"),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_parsing() {
        let maze: Maze = SAMPLE.into();
        assert_eq!(maze.start, (1, 3));
        assert_eq!(maze.end, (5, 7));
        assert_eq!(maze.width, 15);
        assert_eq!(maze.height, 15);
    }

    #[test]
    fn test_shortcut_count() {
        assert_eq!(count_shortcuts(&SAMPLE.into(), 2, 50).unwrap(), 1);
        assert_eq!(count_shortcuts(&SAMPLE.into(), 20, 50).unwrap(), 285);
    }
}
