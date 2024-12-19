use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleResult};

const DAY: Day = Day(14);

#[allow(dead_code)]
pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Restroom Redoubt");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input, 101, 103)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 225943500);

    let p2 = part2(&input, 101, 103)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 6377);

    Ok(())
}

fn part1(input: &Input, width: usize, height: usize) -> PuzzleResult<usize> {
    let mut scene = Scene::parse(width, height, input)?;
    scene.step_n(100);
    Ok(scene.safety_factor())
}

fn part2(input: &Input, width: usize, height: usize) -> PuzzleResult<usize> {
    let scene = Scene::parse(width, height, input)?;
    let (start_v, start_h, period_v, period_h) = find_periods(scene.clone());
    let iteration = find_matching_iteration(start_v, start_h, period_v, period_h);
    assert!(is_xmas_tree(scene, iteration));

    Ok(iteration)
}

fn is_xmas_tree(mut scene: Scene, seconds: usize) -> bool {
    scene.step_n(seconds);
    scene.detect_blob() == BlobDimension::Both
}

fn find_matching_iteration(
    start_v: usize,
    start_h: usize,
    period_v: usize,
    period_h: usize,
) -> usize {
    let mut h = start_h;
    let mut v = start_v;
    while h != v {
        if h < v {
            h += period_h;
        } else {
            v += period_v;
        }
    }

    v
}

fn find_periods(mut scene: Scene) -> (usize, usize, usize, usize) {
    let mut verticals = Vec::new();
    let mut horizontals = Vec::new();

    while verticals.len() < 2 || horizontals.len() < 2 {
        match scene.detect_blob() {
            BlobDimension::Vertical => verticals.push(scene.seconds),
            BlobDimension::Horizontal => horizontals.push(scene.seconds),
            BlobDimension::Both => {
                verticals.push(scene.seconds);
                horizontals.push(scene.seconds);
            }
            _ => {}
        }
        scene.step_n(1);
    }

    let start_v = verticals[0];
    let start_h = horizontals[0];
    let period_v = verticals[1] - verticals[0];
    let period_h = horizontals[1] - horizontals[0];

    (start_v, start_h, period_v, period_h)
}

type Value = i64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BlobDimension {
    None,
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Robot {
    x: Value,
    y: Value,
    dx: Value,
    dy: Value,
}

impl Robot {
    fn new(position: (Value, Value), velocity: (Value, Value)) -> Self {
        Self {
            x: position.0,
            y: position.1,
            dx: velocity.0,
            dy: velocity.1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scene {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
    seconds: usize,
}

impl Scene {
    fn new(width: usize, height: usize, robots: &[Robot]) -> Self {
        Self {
            width,
            height,
            robots: Vec::from(robots),
            seconds: 0,
        }
    }

    fn parse(width: usize, height: usize, input: &Input) -> PuzzleResult<Self> {
        let line_re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let robots = input
            .lines()?
            .map(|line| {
                let caps = line_re.captures(&line).unwrap();
                let values: Vec<Value> = caps
                    .iter()
                    .skip(1)
                    .flatten()
                    .map(|v| v.as_str().parse().unwrap())
                    .collect();

                Robot::new((values[0], values[1]), (values[2], values[3]))
            })
            .collect::<Vec<_>>();

        Ok(Self::new(width, height, &robots))
    }

    fn step_n(&mut self, n: usize) {
        self.seconds += n;
        let n: Value = n.try_into().unwrap();
        for robot in self.robots.iter_mut() {
            robot.x = (robot.x + n * robot.dx) % self.width as Value;
            robot.y = (robot.y + n * robot.dy) % self.height as Value;

            if robot.x < 0 {
                robot.x += self.width as Value;
            }
            if robot.y < 0 {
                robot.y += self.height as Value;
            }
        }
    }

    fn safety_factor(&self) -> usize {
        let factors = self.safety_factors();
        factors.0 * factors.1 * factors.2 * factors.3
    }

    fn safety_factors(&self) -> (usize, usize, usize, usize) {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        let mid_width = self.width as Value / 2;
        let mid_height = self.height as Value / 2;

        for robot in self.robots.iter() {
            if robot.x < mid_width && robot.y < mid_height {
                top_left += 1;
            } else if robot.x > mid_width && robot.y < mid_height {
                top_right += 1;
            } else if robot.x < mid_width && robot.y > mid_height {
                bottom_left += 1;
            } else if robot.x > mid_width && robot.y > mid_height {
                bottom_right += 1;
            }
        }

        (top_left, top_right, bottom_left, bottom_right)
    }

    fn detect_blob(&self) -> BlobDimension {
        let scale = 4;
        let threshold = 40;

        let mut horizontal = vec![0; self.width.div_ceil(scale)];
        let mut vertical = vec![0; self.height.div_ceil(scale)];

        for robot in &self.robots {
            horizontal[(robot.x as usize) / scale] += 1;
            vertical[(robot.y as usize) / scale] += 1;
        }

        let horizontal: Vec<bool> = horizontal.into_iter().map(|c| c >= threshold).collect();
        let vertical: Vec<bool> = vertical.into_iter().map(|c| c >= threshold).collect();

        let check_window = |vec: &Vec<bool>| -> bool {
            vec.windows(4)
                .any(|window| window.iter().filter(|&&c| c).count() >= 3)
        };

        let horizontal_detected = check_window(&horizontal);
        let vertical_detected = check_window(&vertical);

        match (horizontal_detected, vertical_detected) {
            (true, true) => BlobDimension::Both,
            (true, false) => BlobDimension::Horizontal,
            (false, true) => BlobDimension::Vertical,
            (false, false) => BlobDimension::None,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        const FACTOR: usize = 3;
        const THRESHOLD: usize = 1;

        let mut grid = vec![vec![0; self.width / FACTOR + 1]; self.height / FACTOR + 1];

        for robot in &self.robots {
            grid[robot.y as usize / FACTOR][robot.x as usize / FACTOR] += 1;
        }

        for row in grid {
            println!(
                "{}",
                row.iter()
                    .map(|c| if *c < THRESHOLD { ' ' } else { '*' })
                    .collect::<String>()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_print() {
        let mut scene = Scene::parse(11, 7, &SAMPLE.into()).unwrap();
        scene.print();
        scene.step_n(100);
        println!();
        scene.print();
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into(), 11, 7).unwrap(), 12);
    }
}
