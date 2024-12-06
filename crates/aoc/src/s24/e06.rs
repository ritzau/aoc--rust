use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use itertools::Itertools;
use std::collections::HashSet;
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn part1(input: &str) -> PuzzleResult<usize> {
    let (start, map) = parse(input);
    let ps: HashSet<_> = StepIterator::new(&map, start).map(|(p, _)| p).collect();
    Ok(ps.len())
}

fn part2(input: &str) -> PuzzleResult<usize> {
    let (start, map) = parse(input);
    let ps: HashSet<_> = StepIterator::new(&map, start).map(|(p, _)| p).collect();

    let count = ps
        .iter()
        .filter(|&&pos| pos != start)
        .map(|&pos| {
            let mut map = map.clone();
            let (r, c) = pos;
            map[r as usize][c as usize] = '#';
            let mut it = StepIterator::new(&map, start);
            it.by_ref().for_each(drop);
            if it.is_valid() {
                Some(pos)
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .count();

    Ok(count)
}

#[allow(dead_code)]
fn animate(map: &Vec<Vec<char>>, start: (i32, i32)) {
    let ps: Vec<_> = StepIterator::new(&map, start).map(|(p, _)| p).collect();
    println!(
        "{}",
        map.iter().map(|r| r.iter().collect::<String>()).join("\n")
    );

    let mut count = 0;
    let mut marks = Vec::<(i32, i32)>::new();
    for (r, c) in &ps {
        marks.push((*r, *c));
        while marks.len() > 200 {
            marks.remove(0);
        }

        let mut map = map.clone();
        for (r, c) in &marks {
            map[*r as usize][*c as usize] = 'X';
        }

        if count % 10 == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}",
                map.iter()
                    .map(|r| r
                        .iter()
                        .map(|&c| match c {
                            '.' => ' ',
                            _ => c,
                        })
                        .map(|c| format!("{} ", c))
                        .collect::<String>())
                    .join("\n")
            );
            thread::sleep(Duration::from_millis(20));
        }
        count += 1;
    }

    {
        let mut map = map.clone();
        for (r, c) in &marks {
            map[*r as usize][*c as usize] = 'X';
        }
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "{}",
            map.iter().map(|r| r.iter().collect::<String>()).join("\n")
        );
    }
}

struct StepIterator<'a> {
    map: &'a Vec<Vec<char>>,
    pos: (i32, i32),
    dir: Direction,
    first: bool,
    history: HashSet<((i32, i32), Direction)>,
}

impl<'a> StepIterator<'a> {
    fn new(map: &'a Vec<Vec<char>>, pos: (i32, i32)) -> Self {
        Self {
            map,
            pos,
            dir: Direction::North,
            first: true,
            history: HashSet::new(),
        }
    }

    fn valid(&self, r: i32, c: i32) -> bool {
        (0..(self.map.len() as i32)).contains(&r) && (0..(self.map[0].len() as i32)).contains(&c)
    }

    fn is_valid(&self) -> bool {
        self.valid(self.pos.0, self.pos.1)
    }
}

impl<'a> Iterator for StepIterator<'a> {
    type Item = ((i32, i32), Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.pos;

        if self.first {
            self.first = false;
            return Some((self.pos, self.dir));
        }

        if self.history.contains(&(self.pos, self.dir)) {
            return None;
        }

        self.history.insert((self.pos, self.dir));

        if !self.valid(r, c) {
            None
        } else {
            let (next_r, next_c) = match self.dir {
                Direction::North => (r - 1, c),
                Direction::East => (r, c + 1),
                Direction::South => (r + 1, c),
                Direction::West => (r, c - 1),
            };

            if !self.valid(next_r, next_c) {
                self.pos = (next_r, next_c);
                None
            } else if self.map[next_r as usize][next_c as usize] == '#' {
                self.dir = self.dir.turn();
                self.next()
            } else {
                self.pos = (next_r, next_c);
                Some((self.pos, self.dir))
            }
        }
    }
}

fn parse(input: &str) -> ((i32, i32), Vec<Vec<char>>) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .filter_map(|(r, row)| {
            if let Some(c) = row.iter().position(|&c| c == '^') {
                Some((r as i32, c as i32))
            } else {
                None
            }
        })
        .exactly_one()
        .unwrap();

    (start, map)
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
        let (start, map) = parse(SAMPLE);
        assert_eq!(start, (6, 4));
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
    }

    #[test]
    fn test_iterator() {
        let (start, map) = parse(SAMPLE);
        let ps: HashSet<_> = StepIterator::new(&map, start).map(|(p, _)| p).collect();
        assert_eq!(ps.len(), 41);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE.into()).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE.into()).unwrap(), 6);
    }
}
