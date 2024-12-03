use cache::AocCache;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use std::{fmt, io};

pub mod input;

mod cache;
pub mod s15;
pub mod s24;

pub type PuzzleResult<T> = Result<T, PuzzleError>;
type AoCSolution = fn(&AocCache) -> PuzzleResult<bool>;

#[derive(Debug)]
pub enum PuzzleError {
    IO { msg: String, error: io::Error },
    Input(String),
    Verification(String),
    Solution(String, Box<dyn Error>),
    DownloadFailed(String, Box<dyn Error>),
    Cache(String, Box<dyn Error>),
    Processing(String, Box<dyn Error>),
}

impl Error for PuzzleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn run<T>(seq: T) -> PuzzleResult<()>
where
    T: IntoIterator<Item = AoCSolution>,
{
    #[cfg(feature = "OnlyLastPuzzle")]
    {
        if let Some(f) = seq.into_iter().last() {
            verify(f)?;
            Ok(())
        } else {
            Err(PuzzleError::Input("No puzzles available".into()))
        }
    }

    #[cfg(not(feature = "OnlyLastPuzzle"))]
    {
        for f in seq {
            verify(f)?;
        }

        Ok(())
    }
}

fn verify(f: AoCSolution) -> PuzzleResult<()> {
    let cache = AocCache::default();

    let start = std::time::Instant::now();

    let result = match f(&cache) {
        Ok(false) => Err(PuzzleError::Verification("Verification failed".to_string())),
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

impl From<io::Error> for PuzzleError {
    fn from(error: io::Error) -> Self {
        PuzzleError::IO {
            msg: "IO error occurred".to_string(),
            error,
        }
    }
}

#[derive(Debug)]
pub struct Year(u16);

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Day(u8);

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

fn head(year: Year, day: Day, title: &str) {
    println!();
    println!("-- Advent of Code {} Day {}: {} ---", year.0, day.0, title)
}
