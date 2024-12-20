use aoc::{AocCache, PuzzleError, PuzzleResult, Year};

#[macro_use]
mod macros;

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
mod e13;
mod e14;
mod e15;
mod e16;
mod e17;
mod e18;
mod e19;
mod e20;

const YEAR: Year = Year(2024);

type AoCSolution = fn(&AocCache) -> PuzzleResult<()>;

pub fn solve() -> PuzzleResult<()> {
    run_solutions!(
        e01, e02, e03, e04, e05, e06, e07, e08, e09, e10, e11, e12, e13, e14, e15, e16, e17, e18,
        e19, e20
    )
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
        Err(err) => Err(PuzzleError::Solution(format!(
            "Execution failed: {:?}",
            err
        ))),
        _ => Ok(()),
    };

    println!("Duration: {:.1?}", start.elapsed());

    result
}
