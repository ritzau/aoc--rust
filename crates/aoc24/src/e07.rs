use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, Lines, PuzzleError, PuzzleResult};
use itertools::Itertools;
use rayon::prelude::*;

const DAY: Day = Day(7);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Bridge Repair");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 7579994664753);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 438027111276610);

    Ok(())
}

type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn part1(input: &Input) -> PuzzleResult<Value> {
    Ok(sum_valid_calibrations(
        parse(input.lines()?)?,
        &[Operator::Add, Operator::Mul],
    ))
}

fn part2(input: &Input) -> PuzzleResult<i64> {
    Ok(sum_valid_calibrations(
        parse(input.lines()?)?,
        &[Operator::Add, Operator::Mul, Operator::Concat],
    ))
}

fn sum_valid_calibrations(input: Vec<(Value, Vec<Value>)>, operators: &[Operator]) -> Value {
    input
        .into_par_iter()
        .filter(|(result, values)| eval(0, values, *result, operators))
        .map(|(result, _)| result)
        .sum()
}

fn eval(mut value: Value, values: &[Value], target: Value, ops: &[Operator]) -> bool {
    if value > target {
        return false;
    }

    if let [head, tail @ ..] = values {
        ops.iter().any(|op| {
            let new_value = match op {
                Operator::Add => value + head,
                Operator::Mul => value * head,
                Operator::Concat => {
                    let mut temp_value = *head;
                    while temp_value >= 10 {
                        value *= 10;
                        temp_value /= 10;
                    }
                    value * 10 + head
                }
            };

            eval(new_value, tail, target, ops)
        })
    } else {
        value == target
    }
}

fn parse(lines: Lines) -> PuzzleResult<Vec<(Value, Vec<Value>)>> {
    lines.map(|line| parse_line(&line)).try_collect()
}

fn parse_line(line: &str) -> PuzzleResult<(Value, Vec<Value>)> {
    let (result, operands) = line
        .split_once(":")
        .ok_or_else(|| PuzzleError::Input(format!("Line '{line}' does not contain a colon")))?;

    let result = result
        .parse()
        .map_err(|_| PuzzleError::Input(format!("Result '{result}' is not an i64")))?;

    let operands: Vec<Value> = operands
        .split_whitespace()
        .map(|s| s.parse())
        .try_collect()
        .map_err(|_| PuzzleError::Input(format!("Operands '{operands}' are i64s")))?;

    Ok((result, operands))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("190: 10 19").ok(), Some((190, vec![10, 19])));
        assert_eq!(parse_line("190: ").ok(), Some((190, vec![])));
        assert!(matches!(
            parse_line("190 10 19"),
            Err(PuzzleError::Input(_))
        ));
        assert!(matches!(
            parse_line("190: X 19"),
            Err(PuzzleError::Input(_))
        ));
    }

    #[test]
    fn test_parse() {
        let parsed = parse(SAMPLE.into()).unwrap();
        assert_eq!(
            parsed,
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ]
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 11387);
    }
}
