use std::{fs, io, path::PathBuf};

pub mod aoc15e01;

#[derive(Debug)]
struct PuzzleCache {
    root: PathBuf,
}

impl PuzzleCache {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }

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
