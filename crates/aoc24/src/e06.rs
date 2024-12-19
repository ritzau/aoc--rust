use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleResult};
use fxhash::FxHashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::Write;
use std::thread;
use std::time::Duration;

const DAY: Day = Day(6);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Guard Gallivant");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input.read_to_string()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 4665);

    let p2 = part2(&input.read_to_string()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 1688);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Direction(u8);

const DIRECTION_NORTH_VALUE: u8 = 1;
const DIRECTION_EAST_VALUE: u8 = 1 << 1;
const DIRECTION_SOUTH_VALUE: u8 = 1 << 2;
const DIRECTION_WEST_VALUE: u8 = 1 << 3;

const DIRECTION_NORTH: Direction = Direction(DIRECTION_NORTH_VALUE);
const DIRECTION_EAST: Direction = Direction(DIRECTION_EAST_VALUE);
const DIRECTION_SOUTH: Direction = Direction(DIRECTION_SOUTH_VALUE);
const DIRECTION_WEST: Direction = Direction(DIRECTION_WEST_VALUE);

fn turn(direction: Direction) -> Direction {
    match direction {
        DIRECTION_NORTH => DIRECTION_EAST,
        DIRECTION_EAST => DIRECTION_SOUTH,
        DIRECTION_SOUTH => DIRECTION_WEST,
        DIRECTION_WEST => DIRECTION_NORTH,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    OutOfBounds,
    Open,
    Start,
    Obstacle,
}

const GRID_SIZE: usize = 130;

type Visited<const N: usize> = [[u8; N]; N];

fn part1(input: &str) -> PuzzleResult<usize> {
    let (start, map, (row_count, col_count)) = parse::<GRID_SIZE>(input);
    let mut visited = [[0; GRID_SIZE]; GRID_SIZE];
    let ps: FxHashSet<_> = StepIterator::new(&map, &mut visited, row_count, col_count, start)
        .map(|(p, _)| p)
        .collect();

    Ok(ps.len())
}

fn part2(input: &str) -> PuzzleResult<usize> {
    let (start, map, (row_count, col_count)) = parse::<GRID_SIZE>(input);
    let path: Vec<_> = StepIterator::new(
        &map,
        &mut [[0; GRID_SIZE]; GRID_SIZE],
        row_count,
        col_count,
        start,
    )
    .collect();

    let count = path
        .par_iter()
        .map(|&(pos, _)| pos)
        .filter(|&pos| pos != start && creates_loop(&map, row_count, col_count, &path, pos))
        .collect::<FxHashSet<_>>()
        .len();

    Ok(count)
}

fn creates_loop<const N: usize>(
    map: &Grid<N>,
    row_count: usize,
    col_count: usize,
    path: &[((i32, i32), Direction)],
    pos: (i32, i32),
) -> bool {
    // Restart just before first hitting the new obstacle
    // steps > 0 since start is filtered out
    let mut visited = [[0; N]; N];
    let step = path.iter().position(|&(p, _)| p == pos).unwrap();
    for ((r, c), d) in &path[..step - 1] {
        visited[*r as usize][*c as usize] |= d.0;
    }

    let (start, dir) = path[step - 1];
    let mut it = StepIterator::from_state(
        map,
        &mut visited,
        row_count,
        col_count,
        start,
        dir,
        Some((pos.0, pos.1)),
    );
    // Drain the iterator
    it.by_ref().for_each(drop);
    it.is_valid()
}

struct StepIterator<'a, const N: usize> {
    map: &'a Grid<N>,
    visited: &'a mut Visited<N>,
    row_count: usize,
    col_count: usize,
    pos: (i32, i32),
    dir: Direction,
    first: bool,
    extra_obstacle: Option<(i32, i32)>,
}

impl<'a, const N: usize> StepIterator<'a, N> {
    fn new(
        map: &'a Grid<N>,
        visited: &'a mut Visited<N>,
        row_count: usize,
        col_count: usize,
        pos: (i32, i32),
    ) -> Self {
        Self {
            map,
            visited,
            row_count,
            col_count,
            pos,
            dir: DIRECTION_NORTH,
            first: true,
            extra_obstacle: None,
        }
    }

    fn from_state(
        map: &'a Grid<N>,
        visited: &'a mut Visited<N>,
        row_count: usize,
        col_count: usize,
        pos: (i32, i32),
        dir: Direction,
        extra_obstacle: Option<(i32, i32)>,
    ) -> Self {
        Self {
            map,
            visited,
            row_count,
            col_count,
            pos,
            dir,
            first: false,
            extra_obstacle,
        }
    }

    fn valid(&self, r: i32, c: i32) -> bool {
        0 <= r && r < self.row_count as i32 && 0 <= c && c < self.col_count as i32
    }

    fn is_valid(&self) -> bool {
        self.valid(self.pos.0, self.pos.1)
    }

    #[allow(dead_code)]
    fn animate(&mut self) {
        let ps: Vec<_> = self.collect();

        print_map(self.map, self.row_count, self.col_count);

        let mut marks = Vec::<((i32, i32), Direction)>::new();
        for (count, ((r, c), d)) in ps.into_iter().enumerate() {
            marks.push(((r, c), d));
            while marks.len() > 100 {
                marks.remove(0);
            }

            let mut visited = [[0; N]; N];
            for ((r, c), d) in &marks {
                visited[*r as usize][*c as usize] = d.0;
            }

            if count % 2 == 0 {
                print!("\x1B[2J\x1B[1;1H");
                print_map_with_history(self.map, &visited, self.row_count, self.col_count);
                thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

impl<const N: usize> Iterator for StepIterator<'_, N> {
    type Item = ((i32, i32), Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some((self.pos, self.dir));
        }

        let v = &mut self.visited[self.pos.0 as usize][self.pos.1 as usize];
        if (*v & self.dir.0) != 0 {
            return None;
        } else {
            *v |= self.dir.0;
        }

        let (r, c) = self.pos;
        if !self.valid(r, c) {
            return None;
        }

        let (next_r, next_c) = match self.dir {
            DIRECTION_NORTH => (r - 1, c),
            DIRECTION_EAST => (r, c + 1),
            DIRECTION_SOUTH => (r + 1, c),
            DIRECTION_WEST => (r, c - 1),
            _ => unreachable!(),
        };

        if !self.valid(next_r, next_c) {
            self.pos = (next_r, next_c);
            return None;
        }

        if self.map[next_r as usize][next_c as usize] == Tile::Obstacle
            || (self.extra_obstacle == Some((next_r, next_c)))
        {
            self.dir = turn(self.dir);
            return self.next();
        }

        self.pos = (next_r, next_c);
        Some((self.pos, self.dir))
    }
}

type Grid<const N: usize> = [[Tile; N]; N];

fn parse<const N: usize>(input: &str) -> ((i32, i32), Grid<N>, (usize, usize)) {
    let mut start: Option<(i32, i32)> = None;

    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Tile::Open,
                    '^' => {
                        assert_eq!(start, None);
                        start = Some((r as i32, c as i32));
                        Tile::Start
                    }
                    '#' => Tile::Obstacle,
                    _ => Tile::OutOfBounds,
                })
                .collect()
        })
        .collect();

    let mut grid = [[Tile::OutOfBounds; N]; N];
    for (r, row) in map.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            grid[r][c] = tile;
        }
    }

    (start.unwrap(), grid, (map.len(), map[0].len()))
}

#[allow(dead_code)]
fn print_map<const N: usize>(map: &Grid<N>, row_count: usize, col_count: usize) {
    println!(
        "{}",
        map.iter()
            .take(row_count)
            .map(|r| r
                .iter()
                .take(col_count)
                .map(|&c| match c {
                    Tile::OutOfBounds => ' ',
                    Tile::Open => '.',
                    Tile::Start => '^',
                    Tile::Obstacle => '#',
                })
                .fold(String::new(), |mut acc, c| {
                    write!(acc, "{} ", c).unwrap();
                    acc
                }))
            .join("\n")
    );
}

#[allow(dead_code)]
fn print_map_with_history<const N: usize>(
    map: &Grid<N>,
    visited: &Visited<N>,
    row_count: usize,
    col_count: usize,
) {
    println!(
        "{}",
        map.iter()
            .zip(visited.iter())
            .take(row_count)
            .map(|(r, v)| r
                .iter()
                .zip(v.iter())
                .take(col_count)
                .map(|(m, v)| match m {
                    Tile::OutOfBounds => ' ',
                    Tile::Open => {
                        match *v {
                            0 => ' ',
                            DIRECTION_NORTH_VALUE => '^',
                            DIRECTION_EAST_VALUE => '>',
                            DIRECTION_SOUTH_VALUE => 'v',
                            DIRECTION_WEST_VALUE => '<',
                            _ => 'X',
                        }
                    }
                    Tile::Start => {
                        if *v == 0 {
                            '^'
                        } else {
                            'X'
                        }
                    }
                    Tile::Obstacle => '#',
                })
                .fold(String::new(), |mut acc, c| {
                    write!(acc, "{} ", c).unwrap();
                    acc
                }))
            .join("\n")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const SAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_parse() {
        let (start, _map, (row_count, col_count)) = parse::<16>(SAMPLE);
        assert_eq!(start, (6, 4));
        assert_eq!(row_count, 10);
        assert_eq!(col_count, 10);
    }

    #[test]
    fn test_iterator() {
        let (start, map, (row_count, col_count)) = parse::<16>(SAMPLE);
        let mut visited = [[0; 16]; 16];
        let ps: HashSet<_> = StepIterator::new(&map, &mut visited, row_count, col_count, start)
            .map(|(p, _)| p)
            .collect();
        assert_eq!(ps.len(), 41);
    }

    #[test]
    #[ignore] // Only test manually
    fn test_print() {
        let (_, map, (row_count, col_count)) = parse::<16>(SAMPLE);
        print_map(&map, row_count, col_count);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 6);
    }
}
