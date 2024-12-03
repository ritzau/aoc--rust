use crate::{AocCache, Day, PuzzleResult, Year};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

#[derive(Clone)]
enum Source<'a> {
    File(PathBuf),
    #[allow(dead_code)]
    String(&'a str),
}

pub struct Input<'a> {
    implementation: Source<'a>,
}

impl<'a> Input<'a> {
    pub fn from_path(path: PathBuf) -> Self {
        Input {
            implementation: Source::File(path),
        }
    }

    pub fn lines(&self) -> Lines<'a> {
        match &self.implementation {
            Source::File(file) => Lines::from_file(File::open(file).unwrap()),
            Source::String(string) => Lines::from_string(string),
        }
    }

    pub fn read_to_string(&self) -> PuzzleResult<String> {
        match &self.implementation {
            Source::File(path) => {
                let mut reader = BufReader::new(File::open(path)?);
                let mut buffer = String::new();
                reader.read_to_string(&mut buffer)?;
                Ok(buffer)
            }
            Source::String(string) => Ok(string.to_string()),
        }
    }
}

pub struct Lines<'a> {
    implementation: LinesIteratorImpl<'a>,
}

enum LinesIteratorImpl<'a> {
    File(std::io::Lines<BufReader<File>>),
    String(core::str::Lines<'a>),
}

impl<'a> Lines<'a> {
    pub fn from_file(file: File) -> Self {
        Lines {
            implementation: LinesIteratorImpl::File(BufReader::new(file).lines()),
        }
    }

    pub fn from_string(string: &'a str) -> Self {
        let lines = string.lines();
        Lines {
            implementation: LinesIteratorImpl::String(lines),
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.implementation {
            LinesIteratorImpl::File(lines) => lines.next().map(|line| line.unwrap()),
            LinesIteratorImpl::String(iter) => iter.next().map(|line| line.to_string()),
        }
    }
}

pub trait InputFetcher {
    fn get_input(&self, year: Year, day: Day) -> PuzzleResult<Input>;
}

impl InputFetcher for AocCache {
    fn get_input(&self, year: Year, day: Day) -> PuzzleResult<Input> {
        let path = self.get_path(year.0, day.0)?;
        Ok(Input::from_path(path))
    }
}
