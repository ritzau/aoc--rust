use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, create_dir_all, rename, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use std::{fmt, io};

pub mod input;

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
pub struct AocCache {
    root: PathBuf,
}

impl Default for AocCache {
    fn default() -> Self {
        Self {
            root: PathBuf::from("cache"),
        }
    }
}

impl AocCache {
    fn get_session(&self) -> String {
        let path = self.root.join("session.txt");
        fs::read_to_string(path)
            .expect("Session file not found")
            .trim()
            .to_string()
    }

    pub fn get_path(&self, year: u16, day: u8) -> PuzzleResult<PathBuf> {
        let file_path = self.path(year, day);
        let tmp_file_path = format!("{}.tmp", file_path.display());

        // Check if the file already exists, return the stream from the file if it does
        if file_path.is_file() {
            return Ok(file_path);
        }

        // If file doesn't exist, download it to the .tmp file
        println!("File not found, downloading input.");

        let session = self.get_session();

        if let Some(parent) = PathBuf::from(&tmp_file_path).parent() {
            create_dir_all(parent).map_err(|e| {
                PuzzleError::Cache(
                    format!("Failed to create cache directory {}: {e}", parent.display()),
                    e.into(),
                )
            })?;
        }

        // Open the .tmp file for writing
        let mut tmp_file = File::create(&tmp_file_path).map_err(|e| {
            PuzzleError::Cache(
                format!("Failed to open file at {}: {}", tmp_file_path, e),
                e.into(),
            )
        })?;

        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        // Fetch input via streaming
        let response = ureq::get(&url)
            .set("Cookie", &format!("session={}", session))
            .call()
            .map_err(|e| {
                PuzzleError::DownloadFailed(format!("Failed to download {url}: {e}"), e.into())
            })?;

        // Stream the response into the .tmp file
        let mut reader = response.into_reader();
        let mut buffer = [0; 8192]; // 8 KB chunks

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break; // EOF reached
            }
            tmp_file.write_all(&buffer[..bytes_read]).map_err(|e| {
                PuzzleError::Cache(
                    format!("Can't write to file {}: {e}", tmp_file_path),
                    e.into(),
                )
            })?;
        }

        // Rename the .tmp file to the final file name (this is atomic on most filesystems)
        rename(&tmp_file_path, &file_path).map_err(|e| {
            PuzzleError::Cache(
                format!(
                    "Can't rename {} to {}: {e}",
                    tmp_file_path,
                    file_path.display()
                ),
                e.into(),
            )
        })?;

        Ok(file_path)
    }

    fn path(&self, year: u16, day: u8) -> PathBuf {
        self.root
            .join("aoc")
            .join(format!("{}/{:02}.txt", year, day))
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
