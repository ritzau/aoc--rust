use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};

pub fn elves_look_elves_say(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Elves Look, Elves Say");

    let mut input = input
        .read_to_string()
        .map_err(|_| PuzzleError::Input("foo".into()))?
        .trim()
        .to_string();

    for _ in 0..40 {
        input = look_say(&input)?;
    }

    let len_40 = input.len();
    println!("aoc15e10a: {}", len_40);

    for _ in 0..10 {
        input = look_say(&input)?;
    }

    let len_50 = input.len();
    println!("aoc15e10b: {}", len_50);

    Ok(len_40 == 360154 && len_50 == 5103798)
}

fn look_say(s: impl AsRef<str>) -> PuzzleResult<String> {
    let mut result = String::new();
    let mut chars = s.as_ref().chars();
    let mut count = 1;
    let mut prev_char = chars.next().unwrap();

    for c in chars {
        if c == prev_char {
            count += 1;
        } else {
            result.push_str(count.to_string().as_str());
            result.push(prev_char);
            prev_char = c;
            count = 1
        }
    }
    result.push_str(count.to_string().as_str());
    result.push(prev_char);

    Ok(result)
}
