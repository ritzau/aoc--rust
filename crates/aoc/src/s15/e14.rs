use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

const DAY: Day = Day(14);

pub fn reindeer_olympics(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Reindeer Olympics");

    let input = aoc
        .get_input(YEAR, DAY)?
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Input error: {e}")))?;

    let max_distance = part_1(&input)?;
    println!("aoc15e14a: {}", max_distance);

    let score = part_2(&input)?;
    println!("aoc15e14b: {}", score);

    Ok(max_distance == 2696 && score == 1084)
}

fn part_1(input: &str) -> PuzzleResult<u32> {
    let max_distance = input
        .lines()
        .map(Reindeer::from)
        .map(|reindeer| reindeer.distance_after(2503))
        .fold(0, max);

    Ok(max_distance)
}

fn part_2(input: &str) -> PuzzleResult<u32> {
    let reindeers = input.lines().map(Reindeer::from).collect::<Vec<_>>();

    let mut scores = HashMap::<&str, u32>::new();

    for distance in 1..=2503 {
        let standing: Vec<_> = reindeers
            .iter()
            .map(|r| (r.name(), r.distance_after(distance)))
            .sorted_by(|(_, a), (_, b)| b.cmp(a))
            .collect();

        let (_, winner_score) = *standing.first().unwrap();
        standing
            .into_iter()
            .filter(|(_, score)| *score == winner_score)
            .map(|(name, _)| name)
            .for_each(|name| {
                *scores.entry(name).or_insert(0) += 1;
            });
    }

    Ok(scores.into_values().max().unwrap())
}

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    #[allow(dead_code)] // Used for tests
    fn new(name: String, speed: u32, fly_time: u32, rest_time: u32) -> Self {
        Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
        }
    }

    fn from(s: &str) -> Self {
        let pattern = Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
        )
        .unwrap();

        if let Some(caps) = pattern.captures(s) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let speed = caps.get(2).unwrap().as_str().parse().unwrap();
            let fly_time = caps.get(3).unwrap().as_str().parse().unwrap();
            let rest_time = caps.get(4).unwrap().as_str().parse().unwrap();

            Reindeer {
                name,
                speed,
                fly_time,
                rest_time,
            }
        } else {
            panic!("Invalid input: {}", s);
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn distance_after(&self, time: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining = time % cycle_time;

        let fly_time = std::cmp::min(remaining, self.fly_time);
        self.speed * (cycles * self.fly_time + fly_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_inout() {
        let reindeer = Reindeer::from(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        );
        assert_eq!(reindeer, Reindeer::new("Comet".to_string(), 14, 10, 127));

        let reindeer = Reindeer::from(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        assert_eq!(reindeer, Reindeer::new("Dancer".to_string(), 16, 11, 162));
    }

    #[test]
    fn can_calculate_distance() {
        let comet = Reindeer::new("Comet".to_string(), 14, 10, 127);
        assert_eq!(comet.distance_after(1), 14);
        assert_eq!(comet.distance_after(10), 140);
        assert_eq!(comet.distance_after(11), 140);
        assert_eq!(comet.distance_after(1000), 1120);

        let dancer = Reindeer::new("Dancer".to_string(), 16, 11, 162);
        assert_eq!(dancer.distance_after(1), 16);
        assert_eq!(dancer.distance_after(10), 160);
        assert_eq!(dancer.distance_after(11), 176);
        assert_eq!(dancer.distance_after(1000), 1056);
    }
}
