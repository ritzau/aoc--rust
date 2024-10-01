use std::result::Result;

use crate::PuzzleCache;

#[allow(clippy::result_large_err)]
pub fn not_quite_lisp() -> Result<bool, ureq::Error> {
    println!("AOC15E01: Not Quite Lisp");

    let cache = PuzzleCache::new("cache".into());
    let body: String = cache.fetch_input(2015, 1)?;
    let floor_count = count_floors(body.as_str());
    let steps = find_basement(body.as_str()).unwrap();
    println!("aoc15e01a: {}", floor_count);
    println!("aoc15e01b: {}", steps);

    Ok(floor_count == 232 && steps == 1783)
}

fn count_floors(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            otherwise => panic!("Unkown char: {}", otherwise),
        })
        .sum()
}

fn find_basement(input: &str) -> Option<usize> {
    let floors: Vec<i32> = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => panic!("Unkown char: {}", c),
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
