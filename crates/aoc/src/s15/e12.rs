use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};
use regex::Regex;
use std::iter::Peekable;
use std::str::Chars;

pub fn js_abacus_framework_io(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "JSAbacusFramework.io");
    let input = input
        .read_to_string()
        .map_err(|_| PuzzleError::Input("foo".into()))?
        .trim()
        .to_string();

    let sum = sum_numbers(&input)?;
    println!("aoc15e12a: {sum}");

    let red_sum = dummy_parse(&input)?;
    println!("aoc15e12b: {red_sum}");

    Ok(sum == 191164 && red_sum == 87842)
}

fn sum_numbers(input: &String) -> Result<i64, PuzzleError> {
    let non_digit_pattern =
        Regex::new(r"[^-\d]+").map_err(|e| PuzzleError::Input(format!("Bad regex: {e}")))?;

    let sum: i64 = non_digit_pattern
        .split(&input)
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<i64>().unwrap_or_default())
        .sum();

    Ok(sum)
}

struct DummyParser<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> DummyParser<'a> {
    fn new(s: &'a str) -> DummyParser<'a> {
        DummyParser {
            iter: s.chars().peekable(),
        }
    }

    fn accept_if(&mut self, c: char) -> bool {
        self.iter.next_if_eq(&c).is_some()
    }

    fn accept(&mut self) -> Option<char> {
        self.iter.next()
    }

    fn parse(&mut self) -> PuzzleResult<i32> {
        if self.iter.peek().is_none() {
            Ok(0)
        } else if self.accept_if('{') {
            self.parse_object()
        } else if self.accept_if('[') {
            self.parse_array()
        } else {
            Err(PuzzleError::Input("Expected object or array".to_string()))
        }
    }

    fn parse_object(&mut self) -> PuzzleResult<i32> {
        let mut sum = 0;
        let mut is_red = false;

        while self.iter.peek().is_some() {
            if self.accept_if('}') {
                return Ok(if is_red { 0 } else { sum });
            } else if self.accept_if('{') {
                sum += self.parse_object()?;
            } else if self.accept_if('[') {
                sum += self.parse_array()?;
            } else if self.accept_if('"') {
                is_red |= self.parse_string()? == "red";
            } else if self.accept_if('-') {
                sum -= self.parse_number()?;
            } else if self.iter.peek().unwrap().is_digit(10) {
                sum += self.parse_number()?;
            } else {
                self.accept();
            }
        }

        Err(PuzzleError::Input("Unexpected end".to_string()))
    }

    fn parse_array(&mut self) -> PuzzleResult<i32> {
        let mut sum = 0;

        while self.iter.peek().is_some() {
            if self.accept_if(']') {
                return Ok(sum);
            } else if self.accept_if('{') {
                sum += self.parse_object()?;
            } else if self.accept_if('[') {
                sum += self.parse_array()?;
            } else if self.accept_if('"') {
                self.parse_string()?;
            } else if self.accept_if('-') {
                sum -= self.parse_number()?;
            } else if self.iter.peek().unwrap().is_digit(10) {
                sum += self.parse_number()?;
            } else {
                self.accept();
            }
        }

        Err(PuzzleError::Input("Unexpected end".to_string()))
    }

    fn parse_string(&mut self) -> PuzzleResult<String> {
        let mut string = String::new();

        while self.iter.peek().is_some() {
            if self.accept_if('"') {
                return Ok(string);
            } else if let Some(c) = self.accept() {
                string.push(c);
            }
        }

        Err(PuzzleError::Input("Unexpected end".to_string()))
    }

    fn parse_number(&mut self) -> PuzzleResult<i32> {
        let mut number: i32 = 0;
        while let Some(c) = self.iter.peek() {
            if let Some(d) = c.to_digit(10) {
                self.accept();
                number = 10 * number + (d as i32);
            } else {
                return Ok(number);
            }
        }

        Err(PuzzleError::Input("Unexpected end".to_string()))
    }
}

fn dummy_parse(s: impl AsRef<str>) -> PuzzleResult<i32> {
    DummyParser::new(s.as_ref()).parse()
}

#[cfg(test)]
mod test {
    use super::*;
    use log::LevelFilter;
    use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

    #[ctor::ctor]
    fn setup() {
        TermLogger::init(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        )
        .unwrap();
    }

    #[test]
    fn test_json_parser() {
        assert_eq!(dummy_parse(r"").unwrap(), 0);
        assert_eq!(dummy_parse(r"{}").unwrap(), 0);
        assert_eq!(dummy_parse(r"[]").unwrap(), 0);
        assert_eq!(dummy_parse(r"[1,2,3]").unwrap(), 6);
        assert_eq!(dummy_parse(r#"{"a":2,"b":4}"#).unwrap(), 6);
        assert_eq!(dummy_parse(r#"{"a":{"b":4},"c":-1}"#).unwrap(), 3);
        assert_eq!(dummy_parse(r#"{"a":[-1,1]}"#).unwrap(), 0);
        assert_eq!(dummy_parse(r#"[-1,{"a":1}]"#).unwrap(), 0);
    }

    #[test]
    fn test_red() {
        assert_eq!(dummy_parse(r#"[1,2,3]"#).unwrap(), 6);
        assert_eq!(dummy_parse(r#"[1,{"c":"red","b":2},3]"#).unwrap(), 4);
        assert_eq!(
            dummy_parse(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap(),
            0
        );
        assert_eq!(dummy_parse(r#"[1,"red",5]"#).unwrap(), 6);
    }
}
