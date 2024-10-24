use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};
use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn knights_of_the_dinner_table(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Knights of the Dinner Table");

    let p1 = part_1b(input.lines()?)?;
    let p2 = part_2b(input.lines()?)?;

    Ok(p1 == 618 && p2 == 601)
}

#[derive(Debug)]
struct StaticStrings {
    set: HashSet<Box<str>>,
}

impl StaticStrings {
    fn new<T>(strings: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let set: HashSet<_> = strings.into_iter().map(|s| Box::from(s.as_ref())).collect();
        Self { set }
    }

    fn get(&self, s: impl AsRef<str>) -> Option<&str> {
        self.set.get(s.as_ref()).map(|b| b.as_ref())
    }
}

#[derive(Debug)]
struct DynamicStrings {
    set: HashSet<Rc<str>>,
}

impl DynamicStrings {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    fn from<T>(strings: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let set: HashSet<_> = strings.into_iter().map(|s| Rc::from(s.as_ref())).collect();
        Self { set }
    }

    fn get(&mut self, s: impl AsRef<str>) -> Rc<str> {
        if self.set.contains(s.as_ref()) {
            self.set.get(s.as_ref()).unwrap().clone()
        } else {
            let rc: Rc<str> = Rc::from(s.as_ref());
            self.set.insert(rc.clone());
            rc
        }
    }
}

fn part_1(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<i64> {
    let entries = parse_entries(lines)?;
    let strings = get_strings(&entries);
    let (mut peeps, pairs) = parse(&strings, &entries)?;

    let last = peeps.pop().unwrap();

    let optimal = peeps
        .iter()
        .copied()
        .permutations(peeps.len())
        .fold(0, |optimal, seating| {
            let v: Vec<_> = seating.into_iter().chain(std::iter::once(last)).collect();
            let &first = v.first().unwrap();
            let &last = v.last().unwrap();

            let sum = sum_happiness(&pairs, v)
                + pairs.get(&(first, last)).unwrap()
                + pairs.get(&(last, first)).unwrap();

            max(optimal, sum)
        });

    Ok(optimal)
}
fn part_1b(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<i64> {
    let mut strings = DynamicStrings::new();
    let entries = parse_entries_b(&mut strings, lines)?;
    let (mut peeps, pairs) = parse_b(&entries)?;

    let last = peeps.pop().unwrap();

    let n = peeps.len();
    let optimal = peeps
        .into_iter()
        .permutations(n)
        .fold(0, |optimal, mut seating| {
            seating.push(last.clone());

            let first = seating.first().unwrap().clone();
            let last = seating.last().unwrap().clone();
            let pair = (first, last);

            let sum = sum_happiness_b(&pairs, seating)
                + pairs.get(&pair).unwrap()
                + pairs.get(&(pair.1, pair.0)).unwrap();

            max(optimal, sum)
        });

    Ok(optimal)
}

fn part_2(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<i64> {
    let entries = parse_entries(lines)?;
    let strings = get_strings(&entries);
    let (peeps, pairs) = parse(&strings, &entries)?;

    let optimal = peeps
        .iter()
        .copied()
        .permutations(peeps.len())
        .fold(0, |opt, v| max(opt, sum_happiness(&pairs, v)));

    Ok(optimal)
}

fn part_2b(lines: Box<dyn Iterator<Item = PuzzleResult<String>>>) -> PuzzleResult<i64> {
    let mut strings = DynamicStrings::new();
    let entries = parse_entries_b(&mut strings, lines)?;
    let (peeps, pairs) = parse_b(&entries)?;

    let n = peeps.len();
    let optimal = peeps
        .into_iter()
        .permutations(n)
        .fold(0, |opt, v| max(opt, sum_happiness_b(&pairs, v)));

    Ok(optimal)
}

fn sum_happiness(pairs: &HashMap<(&str, &str), i64>, v: Vec<&str>) -> i64 {
    v.iter().tuple_windows().fold(0, |sum, (&p1, &p2)| {
        sum + pairs.get(&(p1, p2)).unwrap() + pairs.get(&(p2, p1)).unwrap()
    })
}

fn sum_happiness_b(pairs: &HashMap<(Rc<str>, Rc<str>), i64>, v: Vec<Rc<str>>) -> i64 {
    v.into_iter().tuple_windows().fold(0, |sum, pair| {
        sum + pairs.get(&pair).unwrap() + pairs.get(&(pair.1, pair.0)).unwrap()
    })
}

fn parse_entries(
    lines: Box<dyn Iterator<Item = PuzzleResult<String>>>,
) -> PuzzleResult<Vec<((String, String), i64)>> {
    Ok(lines.map(|e| parse_line(e.unwrap()).unwrap()).collect())
}

fn parse_entries_b(
    strings: &mut DynamicStrings,
    lines: Box<dyn Iterator<Item = PuzzleResult<String>>>,
) -> PuzzleResult<Vec<((Rc<str>, Rc<str>), i64)>> {
    let entries = lines
        .map(|e| {
            let ((p1, p2), happiness) = parse_line(e.unwrap()).unwrap();
            (
                (strings.get(p1.as_str()), strings.get(p2.as_str())),
                happiness,
            )
        })
        .collect();

    Ok(entries)
}

fn get_strings(entries: &Vec<((String, String), i64)>) -> StaticStrings {
    let peeps: Vec<_> = entries
        .iter()
        .flat_map(|((p1, p2), _)| [p1, p2])
        .sorted()
        .dedup()
        .collect();

    StaticStrings::new(&peeps)
}

fn parse<'a>(
    strings: &'a StaticStrings,
    entries: &Vec<((String, String), i64)>,
) -> PuzzleResult<(Vec<&'a str>, HashMap<(&'a str, &'a str), i64>)> {
    let peeps = entries
        .iter()
        .flat_map(|((p1, p2), _)| [p1, p2])
        .sorted()
        .dedup()
        .map(|e| strings.get(e).unwrap())
        .collect();

    let pairs = entries
        .iter()
        .map(|((p1, p2), weight)| {
            let p1 = strings.get(p1).unwrap();
            let p2 = strings.get(p2).unwrap();
            ((p1, p2), *weight)
        })
        .collect();

    Ok((peeps, pairs))
}

fn parse_b(
    entries: &[((Rc<str>, Rc<str>), i64)],
) -> PuzzleResult<(Vec<Rc<str>>, HashMap<(Rc<str>, Rc<str>), i64>)> {
    let peeps = entries
        .iter()
        .flat_map(|((p1, p2), _)| [p1, p2])
        .sorted()
        .dedup()
        .cloned()
        .collect();

    let pairs = entries
        .iter()
        .map(|((p1, p2), weight)| ((p1.clone(), p2.clone()), *weight))
        .collect();

    Ok((peeps, pairs))
}

fn parse_line(s: impl AsRef<str>) -> PuzzleResult<((String, String), i64)> {
    let ws: Vec<_> = s.as_ref().split_whitespace().collect();
    let name_1 = ws[0];
    let lose_gain = ws[2];
    let sign = match lose_gain {
        "gain" => Ok(1),
        "lose" => Ok(-1),
        unknown => Err(PuzzleError::Input(format!("Unknown lose/gain: {unknown}"))),
    }?;
    let amount = ws[3]
        .parse::<i64>()
        .map_err(|e| PuzzleError::Input(format!("Can't parse line: {}", e)))?;
    let name_2 = ws[10].trim_end_matches('.');

    Ok(((name_1.to_string(), name_2.to_string()), sign * amount))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_parsing() {
        let alice = || "Alice".to_string();
        let bob = || "Bob".to_string();
        let carol = || "Carol".to_string();
        let david = || "David".to_string();

        assert_eq!(
            parse_line("Alice would gain 54 happiness units by sitting next to Bob.").unwrap(),
            ((alice(), bob()), 54)
        );
        assert_eq!(
            parse_line("Alice would lose 79 happiness units by sitting next to Carol.").unwrap(),
            ((alice(), carol()), -79)
        );
        assert_eq!(
            parse_line("Alice would lose 2 happiness units by sitting next to David.").unwrap(),
            ((alice(), david()), -2)
        );
        assert_eq!(
            parse_line("Bob would gain 83 happiness units by sitting next to Alice.").unwrap(),
            ((bob(), alice()), 83)
        );
        assert_eq!(
            parse_line("Bob would lose 7 happiness units by sitting next to Carol.").unwrap(),
            ((bob(), carol()), -7)
        );
        assert_eq!(
            parse_line("Bob would lose 63 happiness units by sitting next to David.").unwrap(),
            ((bob(), david()), -63)
        );
        assert_eq!(
            parse_line("Carol would lose 62 happiness units by sitting next to Alice.").unwrap(),
            ((carol(), alice()), -62)
        );
        assert_eq!(
            parse_line("Carol would gain 60 happiness units by sitting next to Bob.").unwrap(),
            ((carol(), bob()), 60)
        );
        assert_eq!(
            parse_line("Carol would gain 55 happiness units by sitting next to David.").unwrap(),
            ((carol(), david()), 55)
        );
        assert_eq!(
            parse_line("David would gain 46 happiness units by sitting next to Alice.").unwrap(),
            ((david(), alice()), 46)
        );
        assert_eq!(
            parse_line("David would lose 7 happiness units by sitting next to Bob.").unwrap(),
            ((david(), bob()), -7)
        );
        assert_eq!(
            parse_line("David would gain 41 happiness units by sitting next to Carol.").unwrap(),
            ((david(), carol()), 41)
        );
    }

    #[test]
    fn test_make_set() {
        let mut strings = HashSet::<Rc<str>>::new();
        let refs = ["foo", "bar", "baz", "bar"];
        for s in refs {
            if let Some(rc) = strings.get(s) {
                println!("Found: {:?}", rc);
            } else {
                let rc = Rc::from(s);
                println!("Inserted: {:?}", rc);
                strings.insert(rc);
            }
        }
        println!("strings: {:?}", strings);
    }
    #[test]
    fn test_make_set_2() {
        let refs = ["foo", "bar", "baz", "bar"];
        let mut strings = Vec::<Box<str>>::new();
        for s in refs.into_iter().unique() {
            strings.push(Box::from(s));
        }
        strings.sort();
        println!("strings: {:?}", strings);

        let mut srefs = Vec::<&str>::new();
        for s in refs {
            let i = strings.binary_search_by(|elm| elm.as_ref().cmp(s)).unwrap();
            println!("Found: [{i}]={:?}", strings[i]);
            srefs.push(strings[i].as_ref());
        }
        println!("srefs: {:?}", srefs);
    }

    #[test]
    fn test_make_set_3() {
        let refs = ["foo", "bar", "baz", "bar"];
        let strings = StaticStrings::new(refs);
        println!("strings: {:?}", strings);

        let mut srefs = Vec::<&str>::new();
        for s in refs {
            let sr = strings.get(s).unwrap();
            srefs.push(sr);
            if std::ptr::eq(sr, s) {
                println!("`sr` and `s` refer to the same `str`");
            } else {
                println!("`sr` and `s` do not refer to the same `str`");
            }
        }
        println!("srefs: {:?}", srefs);
    }
}
