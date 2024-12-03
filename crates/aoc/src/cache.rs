use crate::{PuzzleError, PuzzleResult};
use std::fs;
use std::fs::{create_dir_all, rename, File};
use std::io::{Read, Write};
use std::path::PathBuf;

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

    fn get_session(&self) -> String {
        let path = self.root.join("session.txt");
        fs::read_to_string(path)
            .expect("Session file not found")
            .trim()
            .to_string()
    }
}
