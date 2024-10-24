use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};
use std::collections::BTreeMap;

pub fn aunt_sue(day: u8, input: impl AsRef<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Aunt Sue");

    let sues = parse(&input)?;
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
    println!("aoc15e16a: {:?}", matching_sues.first().unwrap().id);

    Ok(true)
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

fn parse(input: impl AsRef<dyn PuzzleInput>) -> PuzzleResult<Vec<Sue>> {
    input
        .as_ref()
        .lines()?
        .map(|l| {
            l.and_then(|line| {
                Sue::try_from(line).map_err(|e| PuzzleError::Input(format!("Parse error: {e}")))
            })
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
