use crate::cache::AocCache;
use crate::input::{InputFetcher, Lines};
use crate::s15::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};

const DAY: Day = Day(8);

pub fn matchsticks(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Matchsticks");
    let input = aoc.get_input(YEAR, DAY)?;

    let decoded_delta = decode_delta(input.lines()?)?;
    println!("aoc15e08a: {}", decoded_delta);

    let encoded_delta = encode_delta(input.lines()?)?;
    println!("aoc15e08b: {}", encoded_delta);

    Ok(decoded_delta == 1371 && encoded_delta == 2117)
}

fn decode_delta(lines: Lines) -> PuzzleResult<usize> {
    let mut literal_len = 0usize;
    let mut decoded_len = 0usize;

    for line in lines {
        literal_len += line.len();
        decoded_len += decoded_length(&line)?;
    }

    Ok(literal_len - decoded_len)
}

fn encode_delta(lines: Lines) -> PuzzleResult<usize> {
    let mut literal_len = 0usize;
    let mut encoded_len = 0usize;

    for line in lines {
        literal_len += line.len();
        encoded_len += encoded_length(&line);
    }

    Ok(encoded_len - literal_len)
}
fn decoded_length(s: &str) -> PuzzleResult<usize> {
    let mut cs = s.chars();
    let mut expect = |c| {
        let next = cs.next();
        if Some(c) == next {
            Ok(())
        } else {
            Err(PuzzleError::Input(format!(
                "Expected '{c}' got '{:?}'",
                next
            )))
        }
    };

    let mut count = 0usize;

    expect('"')?;
    loop {
        match cs.next() {
            Some('\\') => match cs.next() {
                Some('\\') | Some('"') => count += 1,
                Some('x') => {
                    let x1 = cs.next();
                    let x2 = cs.next();

                    if x1.is_none() || x2.is_none() {
                        return Err(PuzzleError::Input("Hex sequence incomplete".into()));
                    }
                    count += 1;
                }
                _ => return Err(PuzzleError::Input("Invalid escape sequence".into())),
            },
            Some('"') => break,
            None => return Err(PuzzleError::Input("Unterminated string".into())),
            _ => count += 1,
        }
    }

    Ok(count)
}

fn encoded_length(s: &str) -> usize {
    2 + s.chars().fold(0usize, |count, c| match c {
        '"' | '\\' => count + 2,
        _ => count + 1,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decoded_diff() {
        assert_eq!(decoded_length(r#""""#).ok(), Some(0));
        assert_eq!(decoded_length(r#""abc""#).ok(), Some(3));
        assert_eq!(decoded_length(r#""aaa\"aaa""#).ok(), Some(7));
        assert_eq!(decoded_length(r#""\x27""#).ok(), Some(1));
    }

    #[test]
    fn test_encoded_diff() {
        assert_eq!(encoded_length(r#""""#), 6);
        assert_eq!(encoded_length(r#""abc""#), 9);
        assert_eq!(encoded_length(r#""aaa\"aaa""#), 16);
        assert_eq!(encoded_length(r#""\x27""#), 11);
    }
}
