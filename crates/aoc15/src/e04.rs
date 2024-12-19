use crate::YEAR;
use aoc::{head, AocCache, Day, InputFetcher, PuzzleError, PuzzleResult};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use std::sync::{Arc, Mutex};
use std::thread;

const DAY: Day = Day(4);

pub fn the_ideal_stocking_stuffer(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "The Ideal Stocking Stuffer");

    let input = aoc.get_input(YEAR, DAY)?.read_to_string()?;

    let m = find_match_threaded(input.trim(), 5).ok_or(PuzzleError::Solution(
        "No match found for 5 leading zeroes".into(),
    ))?;
    println!("aoc15e04a: {}", m);

    let m2 = find_match_threaded(input.trim(), 6).ok_or(PuzzleError::Solution(
        "No match found for 6 leading zeroes".into(),
    ))?;
    println!("aoc15e04b: {}", m2);

    Ok(m == 117946 && m2 == 3938038)
}

#[allow(dead_code)]
fn find_match(key: &str) -> Option<u32> {
    let mut i = 0u32;
    loop {
        let test_content = format!("{}{}", key, i);
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
fn find_match2(input: &str) -> Option<u32> {
    let mut i = 0u32;
    loop {
        let test_content = format!("{}{}", input, i);
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

#[allow(dead_code)]
fn find_match_x(input: &str, leading_zeroes: usize) -> Option<u32> {
    let mut buffer = String::with_capacity(input.len() + 10); // Preallocate space
    let mut i = 0u32;

    // Calculate how many full bytes we need to check and the remaining bits
    let full_bytes = leading_zeroes / 2; // 2 hex digits per byte
    let remaining_bits = (leading_zeroes % 2) * 4; // 4 bits per hex digit

    loop {
        buffer.clear(); // Reuse buffer instead of creating new strings
        buffer.push_str(input);
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

#[allow(dead_code)]
fn find_match_chunk(
    input: &str,
    leading_zeroes: usize,
    iterator: impl IntoIterator<Item = u32>,
) -> Option<u32> {
    let mut buffer = String::with_capacity(input.len() + 10); // Preallocate space

    // Calculate how many full bytes we need to check and the remaining bits
    let full_bytes = leading_zeroes / 2; // 2 hex digits per byte
    let remaining_bits = (leading_zeroes % 2) * 4; // 4 bits per hex digit

    for i in iterator.into_iter() {
        buffer.clear(); // Reuse buffer instead of creating new strings
        buffer.push_str(input);
        buffer.push_str(&i.to_string()); // Append `i` to the string

        let digest = md5::compute(buffer.as_bytes());

        // Check full bytes
        if digest[..full_bytes].iter().any(|&byte| byte != 0) {
            continue;
        }

        // Check remaining bits (if any)
        if remaining_bits > 0 {
            let mask = 0xFF << (8 - remaining_bits); // Create a mask for the remaining bits
            if digest[full_bytes] & mask != 0 {
                continue;
            }
        }

        return Some(i);
    }

    None
}

#[allow(dead_code)]
fn find_match_rayon(input: &str, leading_zeroes: usize) -> Option<u32> {
    let chunk_size = 10_000u32;

    for i in (0u32..4_000_000).step_by(chunk_size as usize) {
        let matches: Vec<u32> = (i..i + chunk_size)
            .par_bridge()
            .filter_map(|j| quick_test(input, leading_zeroes, j))
            .collect();

        if let Some(v) = matches.into_iter().min() {
            return Some(v);
        }
    }

    None
}

fn quick_test(input: &str, leading_zeroes: usize, value: u32) -> Option<u32> {
    let mut buffer = String::with_capacity(input.len() + 10); // Preallocate space

    // Calculate how many full bytes we need to check and the remaining bits
    let full_bytes = leading_zeroes / 2; // 2 hex digits per byte
    let remaining_bits = (leading_zeroes % 2) * 4; // 4 bits per hex digit

    // buffer.clear(); // Reuse buffer instead of creating new strings
    buffer.push_str(input);
    buffer.push_str(&value.to_string()); // Append `value` to the string

    let digest = md5::compute(buffer.as_bytes());

    // Check full bytes
    if digest[..full_bytes].iter().any(|&byte| byte != 0) {
        return None;
    }

    // Check remaining bits (if any)
    if remaining_bits > 0 {
        let mask = 0xFF << (8 - remaining_bits); // Create a mask for the remaining bits
        if digest[full_bytes] & mask != 0 {
            return None;
        }
    }

    Some(value)
}

fn find_match_threaded(input: &str, leading_zeroes: usize) -> Option<u32> {
    let num_threads = num_cpus::get();
    let chunk_size = 10_000;
    let find =
        |start| find_match_threaded_chunk(input, leading_zeroes, num_threads, chunk_size, start);

    (0..10_000_000)
        .step_by(num_threads * chunk_size)
        .find_map(find)
}

fn find_match_threaded_chunk(
    input: &str,
    leading_zeroes: usize,
    num_threads: usize,
    chunk_size: usize,
    start: u32,
) -> Option<u32> {
    let input = Arc::new(input.to_string());
    let result: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(None));

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let input = Arc::clone(&input);
        let result = Arc::clone(&result);
        let thread_start = start + (thread_id * chunk_size) as u32;
        let thread_end = thread_start + chunk_size as u32;

        let handle = thread::spawn(move || {
            if let Some(value) = find_match_chunk(&input, leading_zeroes, thread_start..thread_end)
            {
                let mut result = result.lock().unwrap();
                if result.is_none() || value < result.unwrap() {
                    *result = Some(value);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();
    *result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore] // Slow test
    fn find_matches() {
        assert_eq!(find_match_x("abcdef", 5), Some(609043));
        assert_eq!(find_match_x("pqrstuv", 5), Some(1048970));
    }

    #[test]
    fn test_rayon() {
        let input = "abcdef";
        let m = find_match_rayon(input, 5);
        assert_eq!(m, Some(609043));
    }
}
