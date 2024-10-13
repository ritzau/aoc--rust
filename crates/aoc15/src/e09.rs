use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn all_in_a_single_night(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "All in a Single Night");

    let shortest = shortest_path(input.lines()?)?;
    println!("aoc15e09a: {}", shortest);

    let longest = longest_path(input.lines()?)?;
    println!("aoc15e09b: {}", longest);

    Ok(shortest == 207 && longest == 804)
}

type Cities = HashSet<String>;
type Distances = HashMap<(String, String), u64>;
type DistanceIter<'a> = Box<dyn Iterator<Item = u64> + 'a>;

fn find_path<F>(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>, f: F) -> PuzzleResult<u64>
where
    F: Fn(DistanceIter) -> Option<u64>,
{
    let (cities, distances) = build_cities(lines)?;
    match f(generate_distances(&cities, &distances)) {
        Some(result) => Ok(result),
        None => Err(PuzzleError::Input("No cities".into())),
    }
}

fn shortest_path(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<u64> {
    find_path(lines, |iter| iter.min())
}

fn longest_path(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<u64> {
    find_path(lines, |iter| iter.max())
}

fn generate_distances<'a>(cities: &'a Cities, distances: &'a Distances) -> DistanceIter<'a> {
    Box::new(cities.iter().permutations(cities.len()).map(move |cs| {
        let pairs: Vec<_> = cs.iter().copied().zip(cs.iter().copied().skip(1)).collect();
        pairs.into_iter().fold(0u64, move |d, (a, b)| {
            d + distances.get(&(a.into(), b.into())).unwrap()
        })
    }))
}

fn build_cities(
    lines: Box<dyn Iterator<Item = PuzzleResult<String>>>,
) -> PuzzleResult<(Cities, Distances)> {
    let mut cities: Cities = HashSet::new();
    let mut distances: Distances = HashMap::new();

    for line in lines {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        assert!(words[1] == "to" && words[3] == "=");

        let a = words[0].to_string();
        let b = words[2].to_string();
        let distance = words[4].parse::<u64>().unwrap();

        cities.insert(a.clone());
        cities.insert(b.clone());
        distances.insert((a.clone(), b.clone()), distance);
        distances.insert((b, a), distance);
    }

    Ok((cities, distances))
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> Box<dyn Iterator<Item = PuzzleResult<String>>> {
        let input = r"London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141";

        Box::new(input.lines().map(|l| Ok(l.to_string())))
    }

    #[test]
    fn test_shortest() {
        let shortest = shortest_path(sample_input());
        assert_eq!(shortest.ok(), Some(605));
    }

    #[test]
    fn test_longest() {
        let longest = longest_path(sample_input());
        assert_eq!(longest.ok(), Some(982));
    }
}
