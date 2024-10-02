use aoc15::{aoc15e01, aoc15e02, PuzzleCache};

fn main() {
    let fs = [
        aoc15e01::not_quite_lisp,
        aoc15e02::i_was_told_there_would_be_no_math,
    ];

    for (day, f) in fs.iter().enumerate() {
        let day = (1 + day).try_into().unwrap();
        verify(day, f);
    }
}

fn verify<F>(day: u8, f: F)
where
    F: Fn(&PuzzleCache, u8) -> Result<bool, ureq::Error>,
{
    let cache = aoc15::PuzzleCache::default();
    assert!(f(&cache, day).unwrap());
}
