use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleError, PuzzleResult};
use fxhash::FxHashMap;

const DAY: Day = Day(11);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Plutonian Pebbles");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 182081);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 216318908621637);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    blink(parse(input)?, 25)
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    blink(parse(input)?, 75)
}

type Value = u64;

fn blink(mut stones: FxHashMap<Value, Value>, n: usize) -> Result<usize, PuzzleError> {
    for _ in 0..n {
        stones = step(&stones);
    }

    Ok(stones.values().map(|&x| x as usize).sum())
}

fn parse(input: &Input) -> PuzzleResult<FxHashMap<Value, Value>> {
    let histogram = input
        .read_to_string()?
        .split_whitespace()
        .map(|w| w.parse::<Value>().unwrap())
        .fold(FxHashMap::default(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    Ok(histogram)
}

fn step(input: &FxHashMap<Value, Value>) -> FxHashMap<Value, Value> {
    fn increase(map: &mut FxHashMap<Value, Value>, k: Value, v: Value) {
        *map.entry(k).or_insert(0) += v;
    }

    let mut result = FxHashMap::default();

    input.iter().for_each(|(&k, &v)| {
        if k == 0 {
            increase(&mut result, 1, v);
            return;
        }

        let len = ((k as f64).log10() as usize) + 1;
        if len % 2 == 0 {
            let divisor = (10 as Value).pow((len / 2) as u32);
            increase(&mut result, k / divisor, v);
            increase(&mut result, k % divisor, v);
            return;
        }

        increase(&mut result, k.checked_mul(2024).unwrap(), v);
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 65601038650482);
    }
}
