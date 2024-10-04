use std::error::Error;
use std::fs::{self, rename, File};
use std::io::{self, BufReader, Read, Write};
use std::path::PathBuf;

pub mod aoc15e01;
pub mod aoc15e02;
pub mod aoc15e03;
pub mod aoc15e04;

pub use aoc15e01::not_quite_lisp as e01_not_quite_lisp;
pub use aoc15e02::i_was_told_there_would_be_no_math as e02_i_was_told_there_would_be_no_math;
pub use aoc15e03::perfectly_spherical_houses_in_a_vacuum as e03_perfectly_spherical_houses_in_a_vacuum;
pub use aoc15e04::the_ideal_stocking_stuffer as e04_the_ideal_stocking_stuffer;

type AoCSolution = fn(u8, &dyn PuzzleInput) -> Result<bool, Box<dyn Error>>;

pub fn run<T>(seq: T)
where
    T: IntoIterator<Item = AoCSolution>,
{
    for (day, f) in seq.into_iter().enumerate() {
        let day = (1 + day).try_into().unwrap();
        verify(day, f);
    }
}

fn verify(day: u8, f: AoCSolution) {
    let cache = PuzzleCache::default();
    let input = PuzzleFileInput::new(cache.path(2015, day));
    assert!(f(day, &input).unwrap());
}

fn header(day: u8, title: impl AsRef<str>) {
    println!();
    println!("-- Day {}: {} ---", day, title.as_ref())
}

pub trait PuzzleInput {
    fn input(&self) -> Result<BufReader<Box<dyn Read>>, Box<dyn Error>>;

    fn read_to_string(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.input()?; // Get the reader from the input
        let mut content = String::new();
        reader.read_to_string(&mut content)?; // Read all content to the string
        Ok(content)
    }
}

#[derive(Debug)]
pub struct PuzzleFileInput {
    path: PathBuf,
}

impl PuzzleInput for PuzzleFileInput {
    fn input(&self) -> Result<BufReader<Box<dyn Read>>, Box<dyn Error>> {
        let file = File::open(&self.path)?;
        Ok(BufReader::new(Box::new(file)))
    }
}

impl PuzzleFileInput {
    fn new(path: PathBuf) -> PuzzleFileInput {
        PuzzleFileInput { path }
    }
}

#[derive(Debug)]
pub struct PuzzleCache {
    root: PathBuf,
}

impl Default for PuzzleCache {
    fn default() -> Self {
        Self {
            root: PathBuf::from("cache"),
        }
    }
}

impl PuzzleCache {
    fn get_session(&self) -> String {
        let path = self.root.join("session.txt");
        fs::read_to_string(path)
            .expect("Session file not found")
            .trim()
            .to_string()
    }

    #[allow(clippy::result_large_err)]
    pub fn fetch_input(&self, year: u16, day: u8) -> Result<String, ureq::Error> {
        if let Ok(body) = self.load(year, day) {
            return Ok(body);
        }

        let session = self.get_session();

        let body =
            ureq::get(format!("https://adventofcode.com/{}/day/{}/input", year, day).as_str())
                .set("Cookie", format!("session={}", session).as_str())
                .call()?
                .into_string()?;

        self.save(year, day, body.as_str())?;

        Ok(body)
    }

    pub fn get_input(
        &self,
        year: u16,
        day: u8,
    ) -> Result<BufReader<Box<dyn Read>>, Box<dyn std::error::Error>> {
        let file_path = self.path(year, day);
        let tmp_file_path = format!("{}.tmp", file_path.display());

        // Check if the file already exists, return the stream from the file if it does
        if let Ok(file) = File::open(&file_path) {
            println!("File found, loading from disk.");
            return Ok(BufReader::new(Box::new(file)));
        }

        // If file doesn't exist, download it to the .tmp file
        println!("File not found, downloading input.");

        let session = self.get_session();

        // Open the .tmp file for writing
        let mut tmp_file = File::create(&tmp_file_path)?;

        // Fetch input via streaming
        let response = ureq::get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .set("Cookie", &format!("session={}", session))
        .call()?;

        // Stream the response into the .tmp file
        let mut reader = response.into_reader();
        let mut buffer = [0; 8192]; // 8 KB chunks

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break; // EOF reached
            }
            tmp_file.write_all(&buffer[..bytes_read])?;
        }

        // Rename the .tmp file to the final file name (this is atomic on most filesystems)
        rename(&tmp_file_path, &file_path)?;

        // After renaming, return the stream for reading from the file
        let file = File::open(&file_path)?;
        Ok(BufReader::new(Box::new(file)))
    }

    fn path(&self, year: u16, day: u8) -> PathBuf {
        self.root
            .join("aoc15")
            .join(format!("{}_{}.txt", year, day))
    }

    fn load(&self, year: u16, day: u8) -> Result<String, io::Error> {
        fs::read_to_string(self.path(year, day))
    }

    fn save(&self, year: u16, day: u8, data: &str) -> Result<(), io::Error> {
        let path = self.path(year, day);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(self.path(year, day), data)
    }
}
