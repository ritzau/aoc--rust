use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::YEAR;
use crate::{head, Day, PuzzleError, PuzzleResult};

const DAY: Day = Day(2);

#[derive(Debug, PartialEq)]
struct Package {
    length: u32,
    width: u32,
    height: u32,
}

impl Package {
    fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    fn parse(text: &str) -> Self {
        let dimensions: Vec<u32> = text.split('x').map(|s| s.parse().unwrap()).collect();
        assert_eq!(dimensions.len(), 3);
        Self::new(dimensions[0], dimensions[1], dimensions[2])
    }

    fn sides(&self) -> Vec<u32> {
        vec![
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ]
    }

    fn area(&self) -> u32 {
        let sides = self.sides();
        let smallest = sides.iter().min().unwrap_or(&0);
        let area = sides.iter().map(|area| 2 * area).sum::<u32>();
        area + smallest
    }

    fn ribbon(&self) -> u32 {
        let mut lengths = vec![self.length, self.width, self.height];
        let cube: u32 = lengths.iter().product();
        lengths.sort();
        lengths.pop();

        2 * lengths.iter().sum::<u32>() + cube
    }
}

pub fn i_was_told_there_would_be_no_math(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "I Was Told there Would Be No Math");
    let input = aoc.get_input(YEAR, DAY)?;

    let body: String = input
        .read_to_string()
        .map_err(|e| PuzzleError::Input(format!("Failed to read the input for day {DAY}: {e}")))?;

    let packages = parse(body.as_str());
    let area: u32 = packages.iter().map(|p| p.area()).sum();
    let ribbon: u32 = packages.iter().map(|p| p.ribbon()).sum();
    println!("aoc15e02a: {}", area);
    println!("aoc15e02b: {}", ribbon);

    Ok(area == 1588178 && ribbon == 3783758)
}

fn parse(body: &str) -> Vec<Package> {
    body.split('\n')
        .filter(|line| !line.is_empty())
        .map(Package::parse)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_correctly() {
        assert_eq!(Package::parse("2x3x4"), Package::new(2, 3, 4));
        assert_eq!(Package::parse("1x1x10"), Package::new(1, 1, 10));
    }

    #[test]
    fn area() {
        assert_eq!(Package::parse("2x3x4").area(), 58);
        assert_eq!(Package::parse("1x1x10").area(), 43);
    }

    #[test]
    fn ribbon() {
        assert_eq!(Package::parse("2x3x4").ribbon(), 34);
        assert_eq!(Package::parse("1x1x10").ribbon(), 14);
    }
}
