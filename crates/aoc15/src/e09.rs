use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, Lines, PuzzleError, PuzzleResult};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DAY: Day = Day(9);

pub fn all_in_a_single_night(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "All in a Single Night");
    let input = aoc.get_input(YEAR, DAY)?;

    let shortest = shortest_path(input.lines()?)?;
    println!("aoc15e09a: {}", shortest);

    let longest = longest_path(input.lines()?)?;
    println!("aoc15e09b: {}", longest);

    Ok(shortest == 207 && longest == 804)
}

type Cities = HashSet<String>;
type Distances = HashMap<(String, String), u64>;
type DistanceIter<'a> = Box<dyn Iterator<Item = u64> + 'a>;

fn find_path<F>(lines: Lines, f: F) -> PuzzleResult<u64>
where
    F: Fn(DistanceIter) -> Option<u64>,
{
    let (cities, distances) = build_cities(lines)?;
    match f(generate_distances(&cities, &distances)) {
        Some(result) => Ok(result),
        None => Err(PuzzleError::Input("No cities".into())),
    }
}

fn shortest_path(lines: Lines) -> PuzzleResult<u64> {
    find_path(lines, |iter| iter.min())
}

fn longest_path(lines: Lines) -> PuzzleResult<u64> {
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

fn build_cities(lines: Lines) -> PuzzleResult<(Cities, Distances)> {
    let mut cities: Cities = HashSet::new();
    let mut distances: Distances = HashMap::new();

    for line in lines {
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

    const INPUT: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

    #[test]
    fn test_shortest() {
        let sample = Lines::from_string(INPUT);
        let shortest = shortest_path(sample);
        assert_eq!(shortest.ok(), Some(605));
    }

    #[test]
    fn test_longest() {
        let sample = Lines::from_string(INPUT);
        let longest = longest_path(sample);
        assert_eq!(longest.ok(), Some(982));
    }
}
