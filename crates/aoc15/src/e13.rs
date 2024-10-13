use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};

pub fn knights_of_the_dinner_table(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Knights of the Dinner Table");
    Ok(true)
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
}
