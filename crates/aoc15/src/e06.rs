use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleResult};
use fancy_regex::Regex;

const DAY: Day = Day(6);

pub fn probably_a_fire_hazard(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Probably a Fire Hazard");
    let input = aoc.get_input(YEAR, DAY)?;

    #[cfg(feature = "EXCLUDE_SLOW_SOLUTIONS")]
    {
        println!("Skipping...");
        return Ok(true);
    }

    #[cfg(not(feature = "EXCLUDE_SLOW_SOLUTIONS"))]
    {
        let mut grid = LightGrid::new();

        for line in input.lines()? {
            let instruction = Instruction::parse(&line);
            match instruction {
                Instruction::TurnOn(tl, br) => grid.turn_on(tl, br),
                Instruction::TurnOff(tl, br) => grid.turn_off(tl, br),
                Instruction::Toggle(tl, br) => grid.toggle(tl, br),
            }
        }

        let count = grid.count_on();
        println!("aoc15e06a: {}", count);

        let mut grid = LightGrid2::new();

        for line in input.lines()? {
            let instruction = Instruction::parse(&line);
            match instruction {
                Instruction::TurnOn(tl, br) => grid.turn_on(tl, br),
                Instruction::TurnOff(tl, br) => grid.turn_off(tl, br),
                Instruction::Toggle(tl, br) => grid.toggle(tl, br),
            }
        }

        let sum = grid.sum();
        println!("aoc15e06b: {}", sum);

        Ok(count == 400410 && sum == 15343601)
    }
}

#[derive(Debug)]

enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        // turn on 0,0 through 999,999
        // toggle 0,0 through 999,0
        // turn off 499,499 through 500,500
        let pattern =
            Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();

        if let Some(captures) = pattern.captures(s.as_ref()).unwrap() {
            let action = captures.get(1).unwrap().as_str();
            let start_x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let start_y = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let end_x = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let end_y = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

            match action {
                "turn on" => Instruction::TurnOn((start_x, start_y), (end_x, end_y)),
                "turn off" => Instruction::TurnOff((start_x, start_y), (end_x, end_y)),
                "toggle" => Instruction::Toggle((start_x, start_y), (end_x, end_y)),
                _ => panic!("Unknown action: {}", action),
            }
        } else {
            panic!("Invalid instruction format");
        }
    }
}

struct LightGrid {
    grid: Box<[[bool; 1000]; 1000]>,
}

impl LightGrid {
    fn new() -> Self {
        LightGrid {
            grid: Box::new([[false; 1000]; 1000]),
        }
    }

    fn count_on(&self) -> usize {
        self.grid.iter().flatten().filter(|&&light| light).count()
    }

    fn turn_on(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                self.grid[x][y] = true;
            }
        }
    }

    fn turn_off(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                self.grid[x][y] = false;
            }
        }
    }

    fn toggle(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                self.grid[x][y] = !self.grid[x][y];
            }
        }
    }
}

struct LightGrid2 {
    grid: Box<[[usize; 1000]; 1000]>,
}

impl LightGrid2 {
    fn new() -> Self {
        LightGrid2 {
            grid: Box::new([[0; 1000]; 1000]),
        }
    }

    fn sum(&self) -> usize {
        self.grid.iter().flatten().sum()
    }

    fn turn_on(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                self.grid[x][y] += 1;
            }
        }
    }

    fn turn_off(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                if self.grid[x][y] > 0 {
                    self.grid[x][y] -= 1;
                }
            }
        }
    }

    fn toggle(&mut self, top_left: (usize, usize), bottom_right: (usize, usize)) {
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                self.grid[x][y] += 2;
            }
        }
    }
}
