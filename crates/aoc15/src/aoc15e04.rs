use crate::{header, PuzzleError, PuzzleInput, PuzzleResult, EXCLUDE_SLOW_SOLUTIONS};

pub fn the_ideal_stocking_stuffer(day: u8, input: Box<dyn PuzzleInput>) -> PuzzleResult<bool> {
    header(day, "The Ideal Stocking Stuffer");

    if EXCLUDE_SLOW_SOLUTIONS {
        return Ok(true);
    }
    let input = input
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Failed to read the input for day {day}: {e}")))?;

    let m = find_match_x(input.trim(), 5);
    println!("aoc15e04a: {}", m.unwrap());

    let m2 = find_match_x(input.trim(), 6);
    println!("aoc15e04b: {}", m2.unwrap());

    Ok(true)
}

#[allow(dead_code)]
fn find_match(key: impl AsRef<str>) -> Option<u32> {
    let mut i = 0u32;
    loop {
        let test_content = format!("{}{}", key.as_ref(), i);
        let digest = md5::compute(test_content.as_bytes());
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            return Some(i);
        }

        i += 1;

        if i > 10000000 {
            return None;
        }
    }
}

#[allow(dead_code)]
fn find_match2(input: impl AsRef<str>) -> Option<u32> {
    let mut i = 0u32;
    loop {
        let test_content = format!("{}{}", input.as_ref(), i);
        let digest = md5::compute(test_content.as_bytes());
        let s = format!("{:x}", digest);
        if s.starts_with("000000") {
            return Some(i);
        }

        i += 1;

        if i > 10000000 {
            return None;
        }
    }
}

fn find_match_x(input: impl AsRef<str>, leading_zeroes: usize) -> Option<u32> {
    let input_str = input.as_ref();
    let mut buffer = String::with_capacity(input_str.len() + 10); // Preallocate space
    let mut i = 0u32;

    // Calculate how many full bytes we need to check and the remaining bits
    let full_bytes = leading_zeroes / 2; // 2 hex digits per byte
    let remaining_bits = (leading_zeroes % 2) * 4; // 4 bits per hex digit

    loop {
        buffer.clear(); // Reuse buffer instead of creating new strings
        buffer.push_str(input_str);
        buffer.push_str(&i.to_string()); // Append `i` to the string

        let digest = md5::compute(buffer.as_bytes());

        // Check full bytes
        if digest[..full_bytes].iter().any(|&byte| byte != 0) {
            i = i.checked_add(1)?;
            continue;
        }

        // Check remaining bits (if any)
        if remaining_bits > 0 {
            let mask = 0xFF << (8 - remaining_bits); // Create a mask for the remaining bits
            if digest[full_bytes] & mask != 0 {
                i = i.checked_add(1)?;
                continue;
            }
        }

        return Some(i);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_matches() {
        assert_eq!(find_match_x("abcdef", 5), Some(609043));
        assert_eq!(find_match_x("pqrstuv", 5), Some(1048970));
    }
}
