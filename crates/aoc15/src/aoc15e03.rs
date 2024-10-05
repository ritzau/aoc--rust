use std::{collections::HashSet, error::Error, iter};

use crate::{header, PuzzleInput};

pub fn perfectly_spherical_houses_in_a_vacuum(
    day: u8,
    input: &dyn PuzzleInput,
) -> Result<bool, Box<dyn Error>> {
    header(day, "Perfectly Spherical Houses in a Vacuum");

    let input = input.read_to_string()?;
    let house_count = walk(&input);
    println!("aoc15e03a: {}", house_count);

    let robo_count = walk_with_robo(&input);
    println!("aoc15e03b: {}", robo_count);

    Ok(house_count == 2565 && robo_count == 2639)
}

fn walk(input: impl AsRef<str>) -> usize {
    houses_visited(input).len()
}

fn walk_with_robo(input: impl AsRef<str>) -> usize {
    let (santa, robo) = split_work(input);
    let santa_houses = houses_visited(santa);
    let robo_houses = houses_visited(robo);
    santa_houses.union(&robo_houses).count()
}

fn houses_visited(input: impl AsRef<str>) -> HashSet<(i32, i32)> {
    let start = (0, 0);

    input
        .as_ref()
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

fn split_work(input: impl AsRef<str>) -> (String, String) {
    let (even, odd): (Vec<_>, _) = input
        .as_ref()
        .chars()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

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
