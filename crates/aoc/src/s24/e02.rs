use crate::input::{InputFetcher, Lines};
use crate::s24::YEAR;
use crate::{head, AocCache, Day, PuzzleResult};

const DAY: Day = Day(2);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Red-Nosed Reports");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(input.lines())?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 269);

    let p2 = part2(input.lines())?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 337);

    Ok(())
}

fn part1(lines: Lines) -> PuzzleResult<i32> {
    Ok(lines
        .map(|line| get_levels(&line))
        .filter(|levels| test_levels(levels))
        .count() as i32)
}

fn part2(lines: Lines) -> PuzzleResult<i32> {
    Ok(lines
        .map(|line| get_levels(&line))
        .filter(|levels| {
            test_levels(levels)
                || (0..levels.len()).any(|i| {
                    let mut ls = levels.clone();
                    ls.remove(i);
                    test_levels(&ls)
                })
        })
        .count() as i32)
}

fn get_levels(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn test_levels(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect();

    (diffs.iter().all(|&x| x < 0) || diffs.iter().all(|&x| x > 0))
        && diffs.iter().all(|&x| (1..=3).contains(&x.abs()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::Lines;

    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        let lines = Lines::from_string(INPUT);
        assert_eq!(part1(lines).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        let lines = Lines::from_string(INPUT);
        assert_eq!(part2(lines).unwrap(), 4);
    }
}
