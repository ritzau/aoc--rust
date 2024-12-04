use crate::cache::AocCache;
use crate::{PuzzleError, PuzzleResult, Year};
use std::time::Duration;

pub mod e01;
pub mod e02;
pub mod e03;
pub mod e04;

const YEAR: Year = Year(2024);

type AoCSolution = fn(&AocCache) -> PuzzleResult<()>;

pub fn solve() -> PuzzleResult<()> {
    run(&[e01::solve, e02::solve, e03::solve, e04::solve])
}

fn run(seq: &[AoCSolution]) -> PuzzleResult<()> {
    for &f in seq {
        verify(f)?;
    }

    Ok(())
}

fn verify(f: AoCSolution) -> PuzzleResult<()> {
    let start = std::time::Instant::now();

    let cache = AocCache::default();

    let result = match f(&cache) {
        Err(err) => Err(PuzzleError::Solution(
            format!("Execution failed: {:?}", err),
            err.into(),
        )),
        _ => Ok(()),
    };

    let duration = start.elapsed();

    println!(
        "Duration: {:?}",
        Duration::from_millis(duration.as_millis() as u64)
    );

    result
}
