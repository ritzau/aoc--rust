use crate::cache::AocCache;
use crate::input::{InputFetcher, Lines};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};

const DAY: Day = Day(1);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Historian Hysteria");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(input.lines()?)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 2430334);

    let p2 = part2(input.lines()?)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 28786472);

    Ok(())
}

fn part1(input: Lines) -> PuzzleResult<i32> {
    let (mut s1, mut s2) = get_lists(input);

    s1.sort();
    s2.sort();

    Ok(s1
        .into_iter()
        .zip(s2)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>())
}

fn part2(input: Lines) -> PuzzleResult<i32> {
    let (s1, s2) = get_lists(input);

    Ok(s1
        .into_iter()
        .map(|x| x * s2.iter().filter(|&&v| v == x).count() as i32)
        .sum())
}

fn get_lists(input: Lines) -> (Vec<i32>, Vec<i32>) {
    let mut s1 = Vec::<i32>::new();
    let mut s2 = Vec::<i32>::new();

    for line in input {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        assert_eq!(line.len(), 2);
        s1.push(line[0]);
        s2.push(line[1]);
    }

    (s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.into()).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.into()).unwrap(), 31);
    }
}
