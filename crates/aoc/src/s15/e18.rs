use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, AocCache, Day, PuzzleError, PuzzleResult};
use std::cmp::min;
use std::fmt::Display;

const DAY: Day = Day(18);

pub fn like_a_gif_for_your_yard(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Like a GIF For Your Yard");

    let input = aoc
        .get_input(YEAR, DAY)?
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Input error: {e}")))?;

    let mut grid = Grid::from(input.as_ref());
    for _ in 0..100 {
        grid.step();
    }

    let part_1_count = grid.count();
    println!("aoc15e18a: {}", part_1_count);

    let mut grid = Grid::from(input.as_ref());
    for _ in 0..100 {
        grid.step_2();
    }

    let part_2_count = grid.count();
    println!("aoc15e18a: {}", part_2_count);

    Ok(part_1_count == 821 && part_2_count == 886)
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn count(&self) -> usize {
        self.grid.iter().flatten().filter(|&&c| c == '#').count()
    }

    fn step(&mut self) {
        let mut next = self.grid.clone();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                next[y][x] = Self::get_next(self.count_neighbours(x, y), cell);
            }
        }

        self.grid = next;
    }

    fn step_2(&mut self) {
        let max_y = self.grid.len() - 1;
        let max_x = self.grid[max_y].len() - 1;

        self.grid[0][0] = '#';
        self.grid[0][max_x] = '#';
        self.grid[max_y][0] = '#';
        self.grid[max_y][max_x] = '#';

        let mut next = self.grid.clone();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                next[y][x] = if self.is_corner(x, y) {
                    '#'
                } else {
                    Self::get_next(self.count_neighbours(x, y), cell)
                };
            }
        }

        self.grid = next;
    }

    fn get_next(neighbours: usize, ch: char) -> char {
        // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
        // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.

        match ch {
            '#' => match neighbours {
                2 | 3 => '#',
                _ => '.',
            },
            '.' => match neighbours {
                3 => '#',
                _ => '.',
            },
            _ => panic!("unexpected char: {}", ch),
        }
    }

    fn is_corner(&self, x: usize, y: usize) -> bool {
        x == 0 && y == 0
            || x == 0 && y == self.grid.len() - 1
            || x == self.grid.len() - 1 && y == 0
            || x == self.grid.len() - 1 && y == self.grid.len() - 1
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        assert!(!self.grid.is_empty());
        assert!(x < self.grid.len());

        let begin_x = if x == 0 { 0 } else { x - 1 };
        let end_x = min(self.grid.len() - 1, x + 1);
        let begin_y = if y == 0 { 0 } else { y - 1 };
        let end_y = min(self.grid[x].len() - 1, y + 1);

        let mut count = 0;
        for y_neighbour in begin_y..=end_y {
            assert!(!self.grid[y].is_empty());
            assert!(y < self.grid[y].len());
            for x_neighbour in begin_x..=end_x {
                if x_neighbour != x || y_neighbour != y {
                    if self.grid[y_neighbour][x_neighbour] == '#' {
                        count += 1;
                    } else {
                    }
                } else {
                }
            }
        }

        count
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::new(grid)
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .grid
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbour_count() {
        let grid = Grid::from("....\n....\n....\n....\n");
        assert_eq!(grid.count_neighbours(0, 0), 0);
        assert_eq!(grid.count_neighbours(0, 3), 0);
        assert_eq!(grid.count_neighbours(3, 0), 0);
        assert_eq!(grid.count_neighbours(3, 3), 0);
    }

    #[test]
    fn test_step() {
        let input = r"
.#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let mut grid = Grid::from(input);

        println!("input:\n{grid}");
        for _ in 0..4 {
            grid.step();
            println!("input:\n{grid}");
        }

        assert_eq!(grid.count(), 4);
    }

    #[test]
    fn test_step_2() {
        let input = r"##.#.#
...##.
#....#
..#...
#.#..#
####.#";

        let mut grid = Grid::from(input);

        println!("input:\n{grid}");
        for _ in 0..5 {
            grid.step_2();
            println!("input:\n{grid}");
        }

        assert_eq!(grid.count(), 17);
    }
}
