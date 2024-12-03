use crate::input::{InputFetcher, Lines};
use crate::s15::YEAR;
use crate::{head, AocCache, Day, PuzzleError, PuzzleResult};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::HashSet;

const DAY: Day = Day(19);

pub fn medicine_for_rudolph(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Medicine for Rudolph");

    let (rules, molecule) = parse(aoc.get_input(YEAR, DAY)?.lines())?;
    // println!("input: {:?}", input);
    let rule_set = rules
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect::<Vec<_>>();

    let ms = create_molecules(&molecule, &rule_set);
    println!("aoc15e19: {}", ms.len());

    let n = min_reductions_bisect_with_persistent_tracking(&rule_set, &molecule.trim());
    println!(
        "aoc15e19b: {}",
        n.map(|x| x.to_string()).unwrap_or("None".to_string())
    );

    Ok(true)
}

fn parse(lines: Lines) -> PuzzleResult<(Vec<(String, String)>, String)> {
    let mut lines = lines.peekable();

    let rules: Vec<_> = lines
        .peeking_take_while(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<_> = line.split(" => ").collect();
            Ok::<(String, String), PuzzleError>((parts[0].to_string(), parts[1].to_string()))
        })
        .try_collect()?;

    if let Some(_) = lines.peek() {
        // assert that it is empty?
        lines.next();
    }

    let molecule = lines.next().unwrap();

    Ok((rules, molecule))
}

#[allow(dead_code)]
fn create_molecules_x(input: &str, rules: &[(&str, &str)]) -> HashSet<String> {
    rules
        .iter()
        .flat_map(|(pattern, replacement)| {
            input
                .match_indices(pattern)
                .map(move |(i, _)| [&input[..i], replacement, &input[i + pattern.len()..]].concat())
        })
        .collect()
}

fn create_molecules(input: &str, rules: &[(&str, &str)]) -> HashSet<String> {
    let mut molecules = HashSet::with_capacity(rules.len() * input.len() / 2); // Estimate capacity

    for &(pattern, replacement) in rules {
        for (i, _) in input.match_indices(pattern) {
            let mut new_molecule =
                String::with_capacity(input.len() - pattern.len() + replacement.len());
            new_molecule.push_str(&input[..i]);
            new_molecule.push_str(replacement);
            new_molecule.push_str(&input[i + pattern.len()..]);
            molecules.insert(new_molecule);
        }
    }

    molecules
}

#[allow(dead_code)]
fn create_baz(rules: &[(&str, &str)], original: &str, wanted: &str) -> u32 {
    let rules = rules.iter().map(|(a, b)| (*b, *a)).collect::<Vec<_>>();
    // let mut seen = BTreeSet::<String>::from([original.to_string()]);
    let mut set = HashSet::<String>::from([original.to_string()]);
    let mut count = 0;

    loop {
        let mut next = HashSet::<String>::new();
        for m in set {
            let r = create_molecules(&m, &rules);
            next.extend(r);
        }
        // next.retain(|m| !seen.contains(m));
        set = next;
        // seen.extend(set.clone());
        if set.contains(wanted) {
            break;
        }
        count += 1;
        println!("count: {} set: {}", count, set.len());
    }

    count
}

#[allow(dead_code)]
fn create_bar(rules: &[(&str, &str)], original: &str, wanted: &str) -> u32 {
    let rules = rules.iter().map(|(a, b)| (*b, *a)).collect::<Vec<_>>();
    let mut set = HashSet::<String>::from([original.to_string()]);
    let mut count = 0;

    loop {
        let mut next = HashSet::with_capacity(set.len() * rules.len()); // Pre-allocate based on current set and rules size
        for m in set.drain() {
            // Drain to avoid cloning `set` and allow reuse
            for &(pattern, replacement) in &rules {
                for (i, _) in m.match_indices(pattern) {
                    let mut new_molecule =
                        String::with_capacity(m.len() - pattern.len() + replacement.len());
                    new_molecule.push_str(&m[..i]);
                    new_molecule.push_str(replacement);
                    new_molecule.push_str(&m[i + pattern.len()..]);
                    next.insert(new_molecule);
                }
            }
        }

        if next.contains(wanted) {
            break;
        }

        set = next; // Move `next` to `set` instead of cloning
        count += 1;
        println!("count: {} set: {}", count, set.len());
    }

    count
}

#[allow(dead_code)]
fn min_reductions_to_e(rules: &[(&str, &str)], target: &str) -> u32 {
    let reversed_rules: Vec<_> = rules.iter().map(|(a, b)| (*b, *a)).collect();
    let mut rng = rand::thread_rng();
    let mut count = 0;

    // Shuffle rules for a randomized search
    let mut molecule = target.to_string();
    while molecule != "e" {
        let mut progress = false;

        for (replacement, pattern) in reversed_rules.choose_multiple(&mut rng, reversed_rules.len())
        {
            if let Some(pos) = molecule.find(replacement) {
                molecule = [
                    &molecule[..pos],
                    pattern,
                    &molecule[pos + replacement.len()..],
                ]
                .concat();

                count += 1;
                progress = true;
                break;
            }
        }

        // If stuck, restart with random ordering of rules
        if !progress {
            molecule = target.to_string();
            count = 0;
        }
    }

    count
}

fn min_reductions_bisect_with_persistent_tracking(
    rules: &[(&str, &str)],
    target: &str,
) -> Option<u32> {
    let reversed_rules: Vec<_> = rules.iter().map(|(a, b)| (*b, *a)).collect();
    let mut low = 1;
    let mut high = 5; // Estimate; adjust as necessary based on problem constraints

    while low < high {
        let mid = (low + high) / 2;

        let mut visited = Vec::new(); // Start fresh for each depth
        println!("low: {} high: {} mid: {}", low, high, mid);
        if depth_limited_search_with_persistent_tracking(&reversed_rules, target, mid, &mut visited)
            .is_some()
        {
            high = mid; // Solution found, try for smaller depth
        } else {
            low = mid + 1; // No solution, increase depth
        }
    }

    let mut visited = Vec::new();
    depth_limited_search_with_persistent_tracking(&reversed_rules, target, low, &mut visited)
}

fn depth_limited_search_with_persistent_tracking(
    rules: &[(&str, &str)],
    molecule: &str,
    depth: u32,
    visited: &mut Vec<String>,
) -> Option<u32> {
    if molecule == "e" {
        return Some(0);
    }
    if depth == 0 {
        return None;
    }

    // If already visited this molecule, skip to avoid loops
    if visited.contains(&molecule.to_string()) {
        return None;
    }

    // Mark this molecule as visited
    visited.push(molecule.to_string());

    for (replacement, pattern) in rules {
        let mut pos = 0;
        while let Some(found) = molecule[pos..].find(replacement) {
            let i = pos + found;
            let new_molecule =
                [&molecule[..i], pattern, &molecule[i + replacement.len()..]].concat();

            if let Some(steps) = depth_limited_search_with_persistent_tracking(
                rules,
                &new_molecule,
                depth - 1,
                visited,
            ) {
                return Some(steps + 1);
            }

            pos = i + 1;
        }
    }

    visited.pop(); // Backtrack, so remove this molecule from the visited path
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_strings() {
        let input = "HOH";
        let rules = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
        assert_eq!(create_molecules(input, &rules).len(), 4);
    }

    #[test]
    fn test_replace_strings_2() {
        let input = "HOHOHO";
        let rules = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
        assert_eq!(create_molecules(input, &rules).len(), 7);
    }

    #[test]
    fn test_generation() {
        // let rules = vec![
        //     ("e", "H"),
        //     ("e", "O"),
        //     ("H", "HO"),
        //     ("H", "OH"),
        //     ("O", "HH"),
        // ];

        // create_foo(&rules, "HOHOHO");
    }
}
