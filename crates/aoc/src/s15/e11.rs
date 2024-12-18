use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleResult};

const DAY: Day = Day(11);

pub fn corporate_policy(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Corporate Policy");

    let input = aoc
        .get_input(YEAR, DAY)?
        .read_to_string()?
        .trim()
        .to_string();

    let next_password = next_pw(&input);
    println!("aoc15e11a: {}", next_password);

    let second_next_password = next_pw(&next_password);
    println!("aoc15e11b: {}", second_next_password);

    Ok(next_password == "cqjxxyzz" && second_next_password == "cqkaabcc")
}

const A_CHAR: u8 = b'a';
const I_CODE: u8 = b'i' - A_CHAR;
const L_CODE: u8 = b'l' - A_CHAR;
const O_CODE: u8 = b'o' - A_CHAR;
const Z_CODE: u8 = b'z' - A_CHAR;

fn increment(pw: &mut [u8]) {
    for elm in pw {
        if *elm < Z_CODE {
            *elm += 1;
            break;
        } else {
            *elm = 0;
        }
    }
}

fn next_pw(s: &str) -> String {
    let mut bytes = to_bytes(s);
    loop {
        increment(&mut bytes);
        if is_valid(&bytes) {
            return from_bytes(&bytes);
        }
    }
}

fn to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().iter().rev().map(|c| c - A_CHAR).collect()
}

fn from_bytes(bytes: &[u8]) -> String {
    String::from_utf8(bytes.iter().map(|b| b + b'a').rev().collect()).unwrap()
}

fn is_valid(bytes: &[u8]) -> bool {
    !has_forbidden_chars(bytes) && has_straight(bytes) && has_two_pairs(bytes)
}

fn has_forbidden_chars(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|&c| c == I_CODE || c == L_CODE || c == O_CODE)
}

fn has_straight(bytes: &[u8]) -> bool {
    bytes
        .windows(3)
        .any(|w| w[0] >= 2 && w[0] - 1 == w[1] && w[1] - 1 == w[2])
}

fn has_two_pairs(bytes: &[u8]) -> bool {
    let mut last = bytes[0];
    let mut pair_count = 0;

    for &b in &bytes[1..] {
        if b == last {
            pair_count += 1;
            last = 255;
        } else {
            last = b;
        }
    }

    pair_count >= 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        let a = 'a';
        let b = char::from_u32(a as u32 + 1).unwrap();
        assert_eq!(b, 'b');
    }

    #[test]
    fn test_is_valid() {
        let bytes = to_bytes("hijklmmn");
        assert!(has_straight(&bytes));
        assert!(has_forbidden_chars(&bytes));

        let bytes = to_bytes("abbceffg");
        assert!(!has_straight(&bytes));
        assert!(has_two_pairs(&bytes));

        let bytes = to_bytes("abbcegjk");
        assert!(!has_two_pairs(&bytes));

        let bytes = to_bytes("ghjaabcc");
        assert!(!has_forbidden_chars(&bytes));
        assert!(has_straight(&bytes));
        assert!(has_two_pairs(&bytes));
        assert!(is_valid(&bytes));
    }

    #[test]
    fn test_next_pw() {
        assert_eq!(next_pw("abcdefgh"), "abcdffaa");
        assert_eq!(next_pw("ghijklmn"), "ghjaabcc");
    }
}
