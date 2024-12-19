use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleResult};
use std::{collections::HashSet, iter};

const DAY: Day = Day(3);

pub fn perfectly_spherical_houses_in_a_vacuum(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Perfectly Spherical Houses in a Vacuum");

    let input = aoc.get_input(YEAR, DAY)?.read_to_string()?;

    let house_count = walk(&input);
    println!("aoc15e03a: {}", house_count);

    let robo_count = walk_with_robo(&input);
    println!("aoc15e03b: {}", robo_count);

    Ok(house_count == 2565 && robo_count == 2639)
}

fn walk(input: &str) -> usize {
    houses_visited(input).len()
}

fn walk_with_robo(input: &str) -> usize {
    let (santa, robo) = split_work(input);
    let santa_houses = houses_visited(&santa);
    let robo_houses = houses_visited(&robo);
    santa_houses.union(&robo_houses).count()
}

fn houses_visited(input: &str) -> HashSet<(i32, i32)> {
    let start = (0, 0);

    input
        .chars()
        .scan(start, |state, ch| {
            let (x, y) = state;
            *state = match ch {
                'v' => (*x, *y + 1),
                '^' => (*x, *y - 1),
                '<' => (*x - 1, *y),
                '>' => (*x + 1, *y),
                _ => panic!("Unknown direction: '{}'", ch),
            };

            Some(*state)
        })
        .chain(iter::once(start))
        .collect()
}

fn split_work(input: &str) -> (String, String) {
    let (even, odd): (Vec<_>, _) = input.chars().enumerate().partition(|(i, _)| i % 2 == 0);

    let to_string = |vec: Vec<(usize, char)>| vec.into_iter().map(|(_, c)| c).collect::<String>();
    (to_string(even), to_string(odd))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn walks_to_the_right_places() {
        assert_eq!(walk(">"), 2);
        assert_eq!(walk("^>v<"), 4);
        assert_eq!(walk("^v^v^v^v^v"), 2);
    }

    #[test]
    fn splitting_work() {
        assert_eq!(split_work("^v"), (String::from("^"), String::from("v"),));
        assert_eq!(
            split_work("^v^v^v^v^v"),
            (String::from("^^^^^"), String::from("vvvvv"),)
        );
    }

    #[test]
    fn robo_count() {
        assert_eq!(walk_with_robo("^v"), 3);
        assert_eq!(walk_with_robo("^>v<"), 3);
        assert_eq!(walk_with_robo("^v^v^v^v^v"), 11);
    }
}
