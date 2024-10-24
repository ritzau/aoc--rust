use crate::{header, PuzzleError, PuzzleInput, PuzzleResult};

pub fn corporate_policy(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "Corporate Policy");

    let input = input
        .read_to_string()
        .map_err(|_| PuzzleError::Input("foo".into()))?
        .trim()
        .to_string();

    let next_password = next_pw(&input);
    println!("aoc15e11a: {}", next_password);

    let second_next_password = next_pw(&next_password);
    println!("aoc15e11b: {}", second_next_password);

    Ok(next_password == "cqjxxyzz" && second_next_password == "cqkaabcc")
}

const A_CHAR: u8 = 'a' as u8;
const I_CODE: u8 = 'i' as u8 - A_CHAR;
const L_CODE: u8 = 'l' as u8 - A_CHAR;
const O_CODE: u8 = 'o' as u8 - A_CHAR;
const Z_CODE: u8 = 'z' as u8 - A_CHAR;

fn increment(pw: &mut Vec<u8>) {
    for ind in 0..pw.len() {
        if pw[ind] < Z_CODE {
            pw[ind] += 1;
            break;
        } else {
            pw[ind] = 0;
        }
    }
}

fn next_pw(s: impl AsRef<str>) -> String {
    let mut bytes = to_bytes(s);
    loop {
        increment(&mut bytes);
        if is_valid(&bytes) {
            return from_bytes(&bytes);
        }
    }
}

fn to_bytes(s: impl AsRef<str>) -> Vec<u8> {
    s.as_ref()
        .as_bytes()
        .iter()
        .rev()
        .map(|c| c - A_CHAR)
        .collect()
}

fn from_bytes(bytes: &Vec<u8>) -> String {
    String::from_utf8(bytes.iter().map(|b| b + ('a' as u8)).rev().collect()).unwrap()
}

fn is_valid(bytes: &Vec<u8>) -> bool {
    !has_forbidden_chars(bytes) && has_straight(bytes) && has_two_pairs(bytes)
}

fn has_forbidden_chars(bytes: &Vec<u8>) -> bool {
    bytes
        .iter()
        .any(|&c| c == I_CODE || c == L_CODE || c == O_CODE)
}

fn has_straight(bytes: &Vec<u8>) -> bool {
    bytes
        .windows(3)
        .any(|w| w[0] >= 2 && w[0] - 1 == w[1] && w[1] - 1 == w[2])
}

fn has_two_pairs(bytes: &Vec<u8>) -> bool {
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
