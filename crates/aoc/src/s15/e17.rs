use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};

const DAY: Day = Day(17);

pub fn no_such_thing_as_too_much(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "No Such Thing as Too Much");

    let input = aoc
        .get_input(YEAR, DAY)?
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Can't read input: {e}")))?
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let combo_count = pack_count(&input, 150);
    println!("aoc15e17a: {:?}", combo_count);

    let combos = pack(&input, 150);
    let min_len = combos.iter().map(|c| c.len()).min().unwrap();
    let min_count = combos.iter().filter(|c| c.len() == min_len).count();
    println!("aoc15e17b: {:?}", min_count);

    Ok(combo_count == 654 && min_count == 57)
}

fn pack_count(containers: &[i32], target: i32) -> i32 {
    if target < 0 || containers.is_empty() {
        return 0;
    }

    (if target == containers[0] { 1 } else { 0 })
        + pack_count(&containers[1..], target - containers[0])
        + pack_count(&containers[1..], target)
}

fn pack(containers: &[i32], target: i32) -> Vec<Vec<i32>> {
    if target < 0 || containers.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();

    if target == containers[0] {
        result.push(vec![containers[0]]);
    }

    for p in pack(&containers[1..], target - containers[0]) {
        let mut r = vec![containers[0]];
        r.extend(p);
        result.push(r);
    }

    for p in pack(&containers[1..], target) {
        result.push(p);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_sample() {
        let input = vec![20, 15, 10, 5, 5];
        assert_eq!(pack_count(&input, 25), 4);
    }
}
