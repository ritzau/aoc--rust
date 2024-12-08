use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};

const DAY: Day = Day(0);

#[allow(dead_code)]
pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Foo");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 0);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 0);

    Ok(())
}

fn part1(_lines: &Input) -> PuzzleResult<i32> {
    Ok(0)
}

fn part2(_lines: &Input) -> PuzzleResult<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 0);
    }
}
