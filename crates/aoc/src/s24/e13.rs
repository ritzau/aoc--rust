use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use itertools::Itertools;

const DAY: Day = Day(13);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Claw Contraption");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 28262);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 101406661266314);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<i64> {
    let cost = parse(input)?
        .into_iter()
        .map(|input| input.solve().unwrap_or(0))
        .sum();

    Ok(cost)
}

fn part2(input: &Input) -> PuzzleResult<i64> {
    const K: i64 = 10_000_000_000_000;

    let cost = parse(input)?
        .into_iter()
        .map(|mut input| {
            input.prize.0 += K;
            input.prize.1 += K;
            input.solve().unwrap_or(0)
        })
        .sum();

    Ok(cost)
}

fn parse(input: &Input) -> PuzzleResult<Vec<ClawContraption>> {
    Ok(input
        .read_to_string()?
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            ClawContraption::parse(
                &chunk
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>())
}

#[derive(Debug)]
struct ClawContraption {
    a_d: (i64, i64),
    b_d: (i64, i64),
    prize: (i64, i64),
}

impl ClawContraption {
    fn parse(lines: &[String]) -> Self {
        assert_eq!(lines.len(), 3);

        fn parse_coordinates(line: &str, x_prefix: &str, y_prefix: &str) -> (i64, i64) {
            let (_, coords) = line.split_once(": ").unwrap();
            let (x_str, y_str) = coords.split_once(", ").unwrap();
            let x = x_str.trim_start_matches(x_prefix).parse().unwrap();
            let y = y_str.trim_start_matches(y_prefix).parse().unwrap();
            (x, y)
        }

        let (a_x, a_y) = parse_coordinates(&lines[0], "X+", "Y+");
        let (b_x, b_y) = parse_coordinates(&lines[1], "X+", "Y+");
        let (price_x, price_y) = parse_coordinates(&lines[2], "X=", "Y=");

        Self {
            a_d: (a_x, a_y),
            b_d: (b_x, b_y),
            prize: (price_x, price_y),
        }
    }

    fn solve(&self) -> Option<i64> {
        let dx1: i64 = self.a_d.0;
        let dx2: i64 = self.b_d.0;
        let dy1: i64 = self.a_d.1;
        let dy2: i64 = self.b_d.1;
        let x: i64 = self.prize.0;
        let y: i64 = self.prize.1;

        let q = (y * dx2 - x * dy2) % (dy1 * dx2 - dy2 * dx1);
        if q == 0 {
            let a = (y * dx2 - x * dy2) / (dy1 * dx2 - dy2 * dx1);
            assert_eq!((x - dx1 * a) % dx2, 0);
            let b = (x - dx1 * a) / dx2;
            Some(3 * a + b)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_ENTITY: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";

    const SAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_parse_entity() {
        let input = ClawContraption::parse(
            &SAMPLE_ENTITY
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );

        assert_eq!(input.a_d, (94, 34));
        assert_eq!(input.b_d, (22, 67));
        assert_eq!(input.prize, (8400, 5400));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 875318608908);
    }
}
