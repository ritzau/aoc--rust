use crate::cache::AocCache;
use crate::input::{Input, InputFetcher, Lines};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn original_values() -> [Operator; 2] {
        [Operator::Add, Operator::Mul]
    }

    fn values() -> [Operator; 3] {
        [Operator::Add, Operator::Mul, Operator::Concat]
    }
}
fn part1(input: &Input) -> PuzzleResult<Value> {
    let input = parse(input.lines()?)?;

    let sum = input
        .into_par_iter()
        .filter_map(|(result, values)| {
            generate_mutations(&Operator::original_values(), values.len() - 1)
                .find(|op| eval(&values, op, result) == Some(result))
                .map(|_| result)
        })
        .sum();

    Ok(sum)
}

fn part2(input: &Input) -> PuzzleResult<i64> {
    let input = parse(input.lines()?)?;

    let sum = input
        .into_par_iter()
        .filter_map(|(result, values)| {
            generate_mutations(&Operator::values(), values.len() - 1)
                .find(|op| eval(&values, op, result) == Some(result))
                .map(|_| result)
        })
        .sum();

    Ok(sum)
}

fn eval(values: &[Value], operators: &[Operator], target: Value) -> Option<Value> {
    assert_eq!(values.len(), operators.len() + 1);

    let mut result = values[0];
    for (value, op) in values.iter().skip(1).zip(operators) {
        match op {
            Operator::Add => result += value,
            Operator::Mul => result *= value,
            Operator::Concat => {
                let mut temp_value = *value;
                while temp_value >= 10 {
                    result *= 10;
                    temp_value /= 10;
                }
                result = result * 10 + value;
            }
        }
        if result > target {
            return None;
        }
    }
    Some(result)
}

fn generate_mutations<T>(values: &[T], length: usize) -> impl Iterator<Item = Vec<T>> + '_
where
    T: Copy,
{
    let iterators = std::iter::repeat(values.iter()).take(length);

    iterators
        .multi_cartesian_product()
        .map(|combo| combo.into_iter().copied().collect())
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
