use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s15::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};
use std::collections::BTreeMap;

const DAY: Day = Day(16);

pub fn aunt_sue(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Aunt Sue");

    let sues = parse(&aoc.get_input(YEAR, DAY)?)?;
    let tape = Sue {
        id: 0,
        props: BTreeMap::from([
            (SueProp::Children, 3),
            (SueProp::Cats, 7),
            (SueProp::Samoyeds, 2),
            (SueProp::Pomeranians, 3),
            (SueProp::Akitas, 0),
            (SueProp::Vizslas, 0),
            (SueProp::Goldfish, 5),
            (SueProp::Trees, 3),
            (SueProp::Cars, 2),
            (SueProp::Perfumes, 1),
        ]),
    };

    let matching_sues: Vec<_> = sues.iter().filter(|&s| tape.matches(s)).collect();
    assert_eq!(matching_sues.len(), 1);
    let matching_sue = matching_sues.first().unwrap();
    println!("aoc15e16a: {:?}", matching_sue.id);

    let really_matching_sues: Vec<_> = sues.iter().filter(|&s| tape.really_matches(s)).collect();
    assert_eq!(really_matching_sues.len(), 1);
    let really_matching_sue = really_matching_sues.first().unwrap();
    println!("aoc15e16b: {:?}", really_matching_sue.id);

    Ok(matching_sue.id == 213 && really_matching_sue.id == 323)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum SueProp {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl<T> From<T> for SueProp
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref() {
            "children" => SueProp::Children,
            "cats" => SueProp::Cats,
            "samoyeds" => SueProp::Samoyeds,
            "pomeranians" => SueProp::Pomeranians,
            "akitas" => SueProp::Akitas,
            "vizslas" => SueProp::Vizslas,
            "goldfish" => SueProp::Goldfish,
            "trees" => SueProp::Trees,
            "cars" => SueProp::Cars,
            "perfumes" => SueProp::Perfumes,
            _ => panic!("Unknown SueProp: {}", value.as_ref()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Sue {
    id: usize,
    props: BTreeMap<SueProp, usize>,
}

impl Sue {
    fn matches(&self, other: &Sue) -> bool {
        self.props
            .iter()
            .all(|(prop, value)| other.props.get(prop).map_or(true, |v| *v == *value))
    }

    fn really_matches(&self, other: &Sue) -> bool {
        self.props.iter().all(|(prop, value)| {
            other.props.get(prop).map_or(true, |v| match prop {
                SueProp::Cats | SueProp::Trees => return *v > *value,
                SueProp::Pomeranians | SueProp::Goldfish => return *v < *value,
                _ => *v == *value,
            })
        })
    }
}

impl<T> From<T> for Sue
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let s = value.as_ref();

        // Sue 474: samoyeds: 0, akitas: 7, pomeranians: 6
        let parts = s.splitn(2, ": ").map(|s| s.trim()).collect::<Vec<_>>();
        let id = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let props = parts[1]
            .split(", ")
            .map(|s| {
                let parts = s.splitn(2, ": ").collect::<Vec<_>>();
                let prop = SueProp::try_from(parts[0]).unwrap();
                let value = parts[1].parse::<usize>().unwrap();
                (prop, value)
            })
            .collect();

        Sue { id, props }
    }
}

fn parse(input: &Input) -> PuzzleResult<Vec<Sue>> {
    input
        .lines()?
        .map(|line| {
            Sue::try_from(line).map_err(|e| PuzzleError::Input(format!("Parse error: {e}")))
        })
        .collect::<PuzzleResult<Vec<_>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_sue() {
        let s = "Sue 474: samoyeds: 0, akitas: 7, pomeranians: 6";
        let sue = Sue::from(s);
        assert_eq!(sue.id, 474);
        assert_eq!(sue.props.len(), 3);
        assert_eq!(sue.props[&SueProp::Samoyeds], 0);
        assert_eq!(sue.props[&SueProp::Akitas], 7);
        assert_eq!(sue.props[&SueProp::Pomeranians], 6);
        assert_eq!(sue.props.get(&SueProp::Cars), None);
    }
}
