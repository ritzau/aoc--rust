use crate::input::InputFetcher;
use crate::s24::YEAR;
use crate::{head, AocCache, Day, PuzzleResult};
use regex::Regex;

const DAY: Day = Day(3);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Mull It Over");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input.read_to_string()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 156388521);

    let p2 = part2(&input.read_to_string()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 75920122);

    Ok(())
}

fn part1(input: &str) -> PuzzleResult<i32> {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Ok(re
        .captures_iter(input)
        .map(|cap| {
            let (_, [a, b]) = cap.extract();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            a * b
        })
        .sum())
}

fn part2(input: &str) -> PuzzleResult<i32> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();

    let (_, sum) = re
        .find_iter(input)
        .fold((true, 0), |(enabled, sum), m| match m.as_str() {
            "do()" => (true, sum),
            "don't()" => (false, sum),
            op => (enabled, sum + if enabled { multiply(op) } else { 0 }),
        });

    Ok(sum)
}

fn multiply(op: &str) -> i32 {
    thread_local! {
        static LAZY_MUL_REGEX: Regex = Regex::new(r"^mul\((\d+),(\d+)\)$").unwrap();
    }

    LAZY_MUL_REGEX.with(|re| {
        let caps = re.captures(op).unwrap();
        let a = caps[1].parse::<i32>().unwrap();
        let b = caps[2].parse::<i32>().unwrap();

        a * b
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
        assert_eq!(part1(sample).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        let sample = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(sample).unwrap(), 48);
    }
}
