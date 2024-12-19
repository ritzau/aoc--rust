use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleResult};

const DAY: Day = Day(1);

#[allow(clippy::result_large_err)]
pub fn not_quite_lisp(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Not Quite Lisp");
    let input = aoc.get_input(YEAR, DAY)?;
    let input = input.read_to_string()?;

    let floor_count = count_floors(&input);
    println!("aoc15e01a: {}", floor_count);

    let steps = find_basement(&input).unwrap();
    println!("aoc15e01b: {}", steps);

    Ok(floor_count == 232 && steps == 1783)
}

fn count_floors(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            otherwise => panic!("Unknown char: {}", otherwise),
        })
        .sum()
}

fn find_basement(input: &str) -> Option<usize> {
    let floors: Vec<i32> = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => panic!("Unknown char: {}", c),
        })
        .scan(0, |state, x| {
            if *state == -1 {
                return None;
            }
            *state += x;
            Some(*state)
        })
        .collect();

    if floors.last() == Some(&-1) {
        Some(floors.len())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_count_floors() {
        assert_eq!(count_floors(""), 0);
        assert_eq!(count_floors("("), 1);
        assert_eq!(count_floors(")"), -1);
        assert_eq!(count_floors("()"), 0);
        assert_eq!(count_floors(")("), 0);
    }

    #[test]
    fn can_find_basement() {
        assert_eq!(find_basement(")"), Some(1));
        assert_eq!(find_basement("(()"), None);
    }
}
