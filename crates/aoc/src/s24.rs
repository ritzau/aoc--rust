use crate::cache::AocCache;
use crate::{PuzzleError, PuzzleResult, Year};

mod e00;
mod e01;
mod e02;
mod e03;
mod e04;
mod e05;
mod e06;
mod e07;
mod e08;
mod e09;
mod e10;
mod e11;
mod e12;

const YEAR: Year = Year(2024);

type AoCSolution = fn(&AocCache) -> PuzzleResult<()>;

pub fn solve() -> PuzzleResult<()> {
    run(&[
        e01::solve,
        e02::solve,
        e03::solve,
        e04::solve,
        e05::solve,
        e06::solve,
        e07::solve,
        e08::solve,
        e09::solve,
        e10::solve,
        e11::solve,
        e12::solve,
    ])
}

fn run(seq: &[AoCSolution]) -> PuzzleResult<()> {
    for &f in seq {
        verify(f)?;
    }

    Ok(())
}

fn verify(f: AoCSolution) -> PuzzleResult<()> {
    let cache = AocCache::default();

    let start = std::time::Instant::now();

    let result = match f(&cache) {
        Err(err) => Err(PuzzleError::Solution(
            format!("Execution failed: {:?}", err),
            err.into(),
        )),
        _ => Ok(()),
    };

    println!("Duration: {:.1?}", start.elapsed());

    result
}
