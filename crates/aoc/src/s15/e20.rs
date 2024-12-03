use crate::cache::AocCache;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleResult};

const DAY: Day = Day(20);

pub fn infinite_elves_and_infinite_houses(_aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Infinite Elves and Infinite Houses");

    let p1 = part_1(33100000);
    println!("aoc15e20a: {}", p1);

    Ok(true)
}

fn part_1(limit: u64) -> u64 {
    (1u64..)
        .map(|n| {
            let score = (1..=n)
                .map(|x| if n % x == 0 { 10 * x } else { 0 })
                .sum::<u64>();

            (n, score)
        })
        .find(|(_, r)| *r >= limit)
        .map(|(n, _)| n)
        .unwrap()
}
