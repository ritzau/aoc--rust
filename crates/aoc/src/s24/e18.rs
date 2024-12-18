use crate::cache::AocCache;
use crate::input::{Input, InputFetcher, Lines};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DAY: Day = Day(18);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "RAM Run");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input, 1024, 71, 71)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 324);

    let p2 = part2(&input, 71, 71)?;
    println!("Part 2: {},{}", p2.0, p2.1);
    assert_eq!(p2, (46, 23));

    Ok(())
}

fn part1(input: &Input, max_bytes: usize, width: usize, height: usize) -> PuzzleResult<Score> {
    let mut grid = Grid::parse(input.lines()?, width, height)?;
    grid.drop_bytes(max_bytes);
    Ok(grid.dijkstra().expect("No path found"))
}

fn part2(input: &Input, width: usize, height: usize) -> PuzzleResult<(usize, usize)> {
    let grid = Grid::parse(input.lines()?, width, height)?;
    let mut low = 0;
    let mut high = grid.coordinates.len();

    while low < high {
        let mut grid = grid.clone();
        let mid = (low + high) / 2;
        grid.drop_bytes(mid);
        if grid.dijkstra().is_none() {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    if low == 0 {
        return Err(PuzzleError::Input("No solution found".into()));
    }

    Ok(grid.coordinates[low - 1])
}

type Score = u16;
const N: usize = 73;
const BYTE_TILE: Score = Score::MAX;
const UNVISITED_EMPTY_TILE: Score = Score::MAX - 1;

#[derive(Clone)]
struct Grid {
    grid: [[Score; N]; N],
    width: usize,
    height: usize,
    goal: (usize, usize),
    coordinates: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(lines: Lines, width: usize, height: usize) -> PuzzleResult<Self> {
        let coordinates = lines
            .map(|line| {
                let (x, y) = line
                    .split_once(",")
                    .ok_or_else(|| PuzzleError::Input("Invalid format".into()))?;
                let x = x
                    .parse::<usize>()
                    .map_err(|_| PuzzleError::Input("Invalid number".into()))?;
                let y = y
                    .parse::<usize>()
                    .map_err(|_| PuzzleError::Input("Invalid number".into()))?;
                Ok::<(usize, usize), PuzzleError>((x, y))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut grid = [[UNVISITED_EMPTY_TILE; N]; N];

        for x in 0..width + 2 {
            grid[0][x] = BYTE_TILE;
            grid[height + 1][x] = BYTE_TILE;
        }

        for row in grid.iter_mut().take(height + 2) {
            row[0] = BYTE_TILE;
            row[width + 1] = BYTE_TILE;
        }

        grid[1][1] = 0;

        Ok(Self {
            grid,
            width,
            height,
            goal: (width, height),
            coordinates,
        })
    }

    fn drop_bytes(&mut self, n: usize) {
        for (x, y) in self.coordinates.iter().take(n) {
            self.grid[*y + 1][*x + 1] = BYTE_TILE;
        }
    }

    fn dijkstra(&mut self) -> Option<Score> {
        let mut queue = BinaryHeap::new();
        for (y, line) in self.grid.iter().enumerate() {
            for (x, score) in line.iter().enumerate() {
                if *score != BYTE_TILE && *score != UNVISITED_EMPTY_TILE {
                    queue.push(Reverse((*score, (x, y))));
                }
            }
        }

        while let Some(Reverse((score, (x, y)))) = queue.pop() {
            if score == UNVISITED_EMPTY_TILE {
                return None;
            }

            let grid_score = self.grid[y][x];
            if grid_score < score {
                continue;
            }

            let new_score = grid_score + 1;
            let new_positions = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for (nx, ny) in new_positions {
                if (nx, ny) == self.goal {
                    self.grid[ny][nx] = new_score;
                    return Some(new_score);
                }
                let new_grid_score = self.grid[ny][nx];
                if new_grid_score == BYTE_TILE {
                    continue;
                }
                if new_score < new_grid_score {
                    self.grid[ny][nx] = new_score;
                    queue.push(Reverse((new_score, (nx, ny))));
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height + 2 {
            for x in 0..self.width + 2 {
                match self.grid[y][x] {
                    BYTE_TILE => print!("#"),
                    UNVISITED_EMPTY_TILE => print!(" "),
                    score => print!("{:1}", score % 10),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into(), 12, 7, 7).unwrap(), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into(), 7, 7).unwrap(), (6, 1));
    }
}
