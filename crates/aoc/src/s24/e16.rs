use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::e16::Direction::{East, West};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
use fxhash::FxHashSet;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use Direction::{North, South};

const DAY: Day = Day(16);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Reindeer Maze");
    let input = aoc.get_input(YEAR, DAY)?;

    let (p1, p2) = part_1_and_2(&input)?;

    println!("Part 1: {}", p1);
    assert_eq!(p1, 72428);

    println!("Part 2: {}", p2);
    assert_eq!(p2, 456);

    Ok(())
}

fn part_1_and_2(input: &Input) -> Result<(Score, usize), PuzzleError> {
    let maze = Maze::<142>::parse(&input.read_to_string()?);
    let reindeer = Reindeer::new(maze.start.0, maze.start.1);
    let (p1, p2) =
        dijkstra(&reindeer, &maze).ok_or(PuzzleError::Solution("No path found".to_string()))?;

    Ok((p1, p2))
}

struct Maze<const N: usize> {
    grid: [[char; N]; N],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl<const N: usize> Maze<N> {
    fn parse(input: &str) -> Self {
        let mut grid = [[' '; N]; N];
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        for (r, line) in input.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                grid[r][c] = ch;
                match ch {
                    'E' => {
                        assert_eq!(end, None, "Multiple ends found in maze");
                        end = Some((r, c));
                    }
                    'S' => {
                        assert_eq!(start, None, "Multiple starts found in maze");
                        start = Some((r, c));
                    }
                    _ => {}
                }
            }
        }

        let height = input.lines().count();
        let width = input
            .lines()
            .map(|line| line.len())
            .max()
            .expect("Empty maze");

        Maze {
            grid,
            width,
            height,
            start: start.expect("No start found in maze"),
            end: end.expect("No end found in maze"),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!("{}", self.grid[r][c]);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_with_path(&self, scores: &ScoreGrid<N>) {
        // Since grid implements Copy, we don't need to clone.
        let mut grid = self.grid;
        scores.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().for_each(|(c, col)| {
                if col.iter().any(|&s| s < Score::MAX) {
                    grid[r][c] = '*';
                }
            });
        });

        for row in grid.iter().take(self.height) {
            for element in row.iter().take(self.width) {
                let tile = match element {
                    '.' => ' ',
                    or => *or,
                };
                print!("{} ", tile);
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl From<Direction> for usize {
    fn from(d: Direction) -> usize {
        match d {
            North => 0,
            South => 1,
            West => 2,
            East => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Reindeer {
    r: usize,
    c: usize,
    direction: Direction,
}

impl Reindeer {
    fn new(r: usize, c: usize) -> Self {
        Reindeer {
            r,
            c,
            direction: East,
        }
    }
}

type Score = u32;
type ScoreGrid<const N: usize> = [[[Score; 4]; N]; N];

#[derive(Debug, Eq, PartialEq)]
struct Step {
    r: usize,
    c: usize,
    direction: Direction,
    score: Score,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<const N: usize>(reindeer: &Reindeer, maze: &Maze<N>) -> Option<(Score, usize)> {
    let try_visit = |r: usize,
                     c: usize,
                     direction: Direction,
                     score: Score,
                     scores: &mut ScoreGrid<N>,
                     queue: &mut BinaryHeap<Step>| {
        if maze.grid[r][c] == '#' || scores[r][c][usize::from(direction)] <= score {
            return;
        }

        scores[r][c][usize::from(direction)] = score;
        queue.push(Step {
            r,
            c,
            direction,
            score,
        });
    };

    let mut queue = BinaryHeap::<Step>::new();
    let mut scores = [[[Score::MAX; 4]; N]; N];
    try_visit(
        reindeer.r,
        reindeer.c,
        reindeer.direction,
        0,
        &mut scores,
        &mut queue,
    );

    while let Some(Step {
        r,
        c,
        direction,
        score,
    }) = queue.pop()
    {
        if scores[r][c][usize::from(direction)] < score {
            continue;
        }

        if maze.grid[r][c] == 'E' {
            let tile_count = backtrace(&scores, maze.end, direction);
            return Some((score, tile_count));
        }

        match direction {
            North => {
                try_visit(r - 1, c, direction, score + 1, &mut scores, &mut queue);
                try_visit(r, c, West, score + 1000, &mut scores, &mut queue);
                try_visit(r, c, East, score + 1000, &mut scores, &mut queue);
            }
            South => {
                try_visit(r + 1, c, direction, score + 1, &mut scores, &mut queue);
                try_visit(r, c, East, score + 1000, &mut scores, &mut queue);
                try_visit(r, c, West, score + 1000, &mut scores, &mut queue);
            }
            West => {
                try_visit(r, c - 1, direction, score + 1, &mut scores, &mut queue);
                try_visit(r, c, South, score + 1000, &mut scores, &mut queue);
                try_visit(r, c, North, score + 1000, &mut scores, &mut queue);
            }
            East => {
                try_visit(r, c + 1, direction, score + 1, &mut scores, &mut queue);
                try_visit(r, c, North, score + 1000, &mut scores, &mut queue);
                try_visit(r, c, South, score + 1000, &mut scores, &mut queue);
            }
        };
    }

    None
}

fn backtrace<const N: usize>(
    scores: &[[[Score; 4]; N]; N],
    end: (usize, usize),
    end_direction: Direction,
) -> usize {
    let mut queue = VecDeque::<(usize, usize, Direction)>::from([(end.0, end.1, end_direction)]);

    let mut path = FxHashSet::<(usize, usize)>::default();

    while let Some((r, c, direction)) = queue.pop_front() {
        let score = scores[r][c][usize::from(direction)];
        path.insert((r, c));
        if score == 0 {
            continue;
        }

        for d in &[North, South, West, East] {
            if *d == direction {
                continue;
            }
            let other_score = scores[r][c][usize::from(*d)];
            if other_score == score - 1000 {
                queue.push_back((r, c, *d));
            }
        }

        let prev_pos = match direction {
            North => (r + 1, c),
            South => (r - 1, c),
            West => (r, c + 1),
            East => (r, c - 1),
        };

        if scores[prev_pos.0][prev_pos.1][usize::from(direction)] == score - 1 {
            queue.push_back((prev_pos.0, prev_pos.1, direction));
        }
    }

    path.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SAMPLE_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    #[ignore] // Manually verify output
    fn test_maze_parse() {
        let maze = Maze::<20>::parse(SAMPLE_1);
        maze.print();
        println!("Start: {:?}", maze.start);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1_and_2(&SAMPLE_1.into()).unwrap().0, 7036);
        assert_eq!(part_1_and_2(&SAMPLE_2.into()).unwrap().0, 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_1_and_2(&SAMPLE_1.into()).unwrap().1, 45);
        assert_eq!(part_1_and_2(&SAMPLE_2.into()).unwrap().1, 64);
    }
}
