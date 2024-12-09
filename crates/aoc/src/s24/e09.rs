use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};
use std::fmt::Display;

const DAY: Day = Day(9);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Disk Fragmenter");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 6367087064415);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 6390781891880);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    let mut d = Disk::from_str(&input.read_to_string()?);
    d.compact();
    Ok(d.checksum())
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    let d = Disk::from_str(&input.read_to_string()?);
    Ok(d.compact_checksum()?)
}

type FileId = u32;

struct Disk(Vec<FileId>);

impl Disk {
    fn from_str(input: &str) -> Self {
        let input = input.trim();

        let cap = input
            .chars()
            .map(|e| e.to_digit(10).unwrap() as usize)
            .sum();

        let mut disk = Vec::<FileId>::with_capacity(cap);
        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                for _ in 0..c.to_digit(10).unwrap() {
                    disk.push((i / 2).try_into().unwrap());
                }
            } else {
                for _ in 0..c.to_digit(10).unwrap() {
                    disk.push(FileId::MAX);
                }
            }
        }

        Disk(disk)
    }

    fn compact(&mut self) {
        let mut a = 0usize;
        let mut b = self.0.iter().len() - 1;

        while a < b {
            if self.0[a] != FileId::MAX {
                a += 1;
            } else if self.0[b] == FileId::MAX {
                b -= 1;
            } else {
                self.0.swap(a, b);
                a += 1;
                b -= 1;
            }
        }
    }

    #[allow(dead_code)]
    fn compact_whole_files(&mut self) {
        let mut free_list = self.build_free_list();
        let mut start: Option<usize> = None;
        let mut last_file_id: Option<FileId> = None;

        for block_index in (0..self.0.len()).rev() {
            let file_id = self.0[block_index];

            match last_file_id {
                Some(last) if file_id != last => {
                    let file_size = start.unwrap() - block_index;
                    Self::move_file(
                        &mut self.0,
                        &mut free_list,
                        last,
                        block_index + 1,
                        file_size,
                    );

                    if file_id != FileId::MAX {
                        start = Some(block_index);
                        last_file_id = Some(file_id);
                    } else {
                        start = None;
                        last_file_id = None;
                    }
                }
                None if file_id != FileId::MAX => {
                    start = Some(block_index);
                    last_file_id = Some(file_id);
                }
                _ => {}
            }
        }
        // Last file found always starts on block 0 and can't be moved
    }

    fn compact_checksum(&self) -> PuzzleResult<usize> {
        let mut free_list = self.build_free_list();
        let mut start: Option<usize> = None;
        let mut last_file_id: Option<FileId> = None;

        let mut checksum: usize = 0;

        for block_index in (0..self.0.len()).rev() {
            let file_id = self.0[block_index];

            match last_file_id {
                Some(last) if file_id != last => {
                    let file_size = start.unwrap() - block_index;
                    checksum +=
                        Self::checksum_file(&mut free_list, last, block_index + 1, file_size)?;

                    if file_id != FileId::MAX {
                        start = Some(block_index);
                        last_file_id = Some(file_id);
                    } else {
                        start = None;
                        last_file_id = None;
                    }
                }
                None if file_id != FileId::MAX => {
                    start = Some(block_index);
                    last_file_id = Some(file_id);
                }
                _ => {}
            }
        }
        if let Some(last) = last_file_id {
            let file_size = start.unwrap() + 1;
            checksum += Self::checksum_file(&mut free_list, last, 0, file_size)?;
        }

        Ok(checksum)
    }

    fn build_free_list(&self) -> Vec<(usize, usize)> {
        let mut free_list = Vec::<(usize, usize)>::new();
        let mut start: Option<usize> = None;

        for (i, e) in self.0.iter().enumerate() {
            if *e == FileId::MAX {
                if start.is_none() {
                    start = Some(i);
                }
            } else {
                if let Some(s) = start {
                    free_list.push((s, i - s));
                    start = None;
                }
            }
        }

        free_list
    }

    fn checksum(&self) -> usize {
        let mut sum = 0usize;
        for (i, &e) in self.0.iter().enumerate() {
            if e < FileId::MAX {
                sum = sum.checked_add(e as usize * i).unwrap();
            }
        }

        sum
    }

    #[allow(dead_code)]
    fn move_file(
        disk: &mut Vec<FileId>,
        free_list: &mut Vec<(usize, usize)>,
        file_id: FileId,
        file_start_index: usize,
        file_size: usize,
    ) {
        let free_list_index = Self::free_entry_position(free_list, file_start_index, file_size);

        if let Some(free_list_index) = free_list_index {
            let dst_start_index = free_list[free_list_index].0;
            for j in 0..file_size {
                disk[dst_start_index + j] = file_id;
                disk[file_start_index + j] = FileId::MAX;
            }
            Self::allocate_block(free_list, free_list_index, file_size);
        }
    }

    fn checksum_file(
        free_list: &mut Vec<(usize, usize)>,
        file_id: FileId,
        file_start_index: usize,
        file_size: usize,
    ) -> PuzzleResult<usize> {
        let free_list_index = Self::free_entry_position(free_list, file_start_index, file_size);
        let start = if let Some(free_list_index) = free_list_index {
            let start = free_list[free_list_index].0;
            Self::allocate_block(free_list, free_list_index, file_size);
            start
        } else {
            file_start_index
        };

        Ok((0..file_size).map(|j| (start + j) * file_id as usize).sum())
    }

    fn free_entry_position(
        free_list: &mut Vec<(usize, usize)>,
        file_start_index: usize,
        file_size: usize,
    ) -> Option<usize> {
        free_list
            .iter()
            .take_while(|(start_block, _)| *start_block < file_start_index)
            .position(|(_, size)| *size >= file_size)
    }

    fn allocate_block(
        free_list: &mut Vec<(usize, usize)>,
        free_list_index: usize,
        file_size: usize,
    ) {
        if let Some(entry) = free_list.get_mut(free_list_index) {
            if entry.1 == file_size {
                free_list.remove(free_list_index);
            } else {
                entry.0 += file_size;
                entry.1 -= file_size;
            }
        }
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &e in self.0.iter() {
            if e < FileId::MAX {
                write!(f, "{:?}", e)?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let d = Disk::from_str(SAMPLE);

        let formatted_output = format!("{}", d);

        let expected_output = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_compact() {
        let mut d = Disk::from_str(SAMPLE);

        d.compact();

        let formatted_output = format!("{}", d);
        let expected_output = "0099811188827773336446555566..............";
        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_checksum() {
        let mut d = Disk::from_str(SAMPLE);

        d.compact();

        assert_eq!(d.checksum(), 1928usize);
    }

    #[test]
    fn test_compact_whole_files() {
        let mut d = Disk::from_str(SAMPLE);

        d.compact_whole_files();

        let formatted_output = format!("{}", d);
        let expected_output = "00992111777.44.333....5555.6666.....8888..";
        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 2858);
    }
}
