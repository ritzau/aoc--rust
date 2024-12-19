use crate::YEAR;
use aoc::{head, AocCache, Day, Input, InputFetcher, PuzzleError, PuzzleResult};
use itertools::Itertools;

const DAY: Day = Day(17);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Chronospatial Computer");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, "1,3,7,4,6,4,2,3,5");

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 202_367_025_818_154);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<String> {
    let mut computer = ChronospatialComputer::parse(&input.read_to_string()?);
    Ok(computer.execute().iter().join(","))
}

fn part2(input: &Input) -> PuzzleResult<Value> {
    let computer = ChronospatialComputer::parse(&input.read_to_string()?);

    fn to_value(cs: &[u8]) -> Value {
        cs.iter().fold(0, |value, &x| (value << 3) | x as Value)
    }

    let run_program = |cs: &[u8]| {
        let mut computer = computer.clone();
        computer.registers[REG_A] = to_value(cs);
        computer.execute()
    };

    let n = computer.opcodes.len();
    let mut codes = vec![0; n];

    let find_code = |cs: &mut [u8], i| {
        let needle = &computer.opcodes[n - i - 1..];
        while cs[i] < 8 {
            if run_program(cs).ends_with(needle) {
                return true;
            }
            cs[i] += 1;
        }
        false
    };

    let mut i = 0;
    while i < n {
        if find_code(&mut codes, i) {
            i += 1;
        } else if i > 0 {
            codes[i] = 0;
            i -= 1;
            codes[i] += 1;
        } else {
            return Err(PuzzleError::Solution("No solution found".into()));
        }
    }

    Ok(to_value(&codes))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction: {}", value),
        }
    }
}

type Value = u64;
const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Clone, Debug)]
struct ChronospatialComputer {
    registers: [Value; 3],
    program: Vec<(Instruction, u8)>,
    opcodes: Vec<u8>,
    program_counter: usize,
}

impl ChronospatialComputer {
    fn parse(input: &str) -> Self {
        let mut registers = [0; 3];

        let (register_lines, program_lines) = input.split_once("\n\n").unwrap();
        register_lines.lines().for_each(|line| {
            let (register, value) = line.split_once(": ").unwrap();
            let register = register.chars().last().unwrap() as usize - 'A' as usize;
            registers[register] = value.parse().unwrap();
        });

        let (_, program) = program_lines.trim().split_once(": ").unwrap();
        let opcodes: Vec<_> = program
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let program = opcodes
            .iter()
            .tuples()
            .map(|(&instruction, &operand)| (instruction.into(), operand))
            .collect();

        Self {
            registers,
            program,
            opcodes,
            program_counter: 0,
        }
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut output = vec![];

        while let Some((instruction, operand)) = self.program.get(self.program_counter) {
            let operand = *operand;

            match instruction {
                // The adv instruction (opcode 0) performs division. The numerator is the value in
                // the A register. The denominator is found by raising 2 to the power of the
                // instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
                // an operand of 5 would divide A by 2^B.) The result of the division operation is
                // truncated to an integer and then written to the A register.
                Instruction::Adv => {
                    self.registers[REG_A] >>= self.combo_value(operand);
                }
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
                // instruction's literal operand, then stores the result in register B.
                Instruction::Bxl => {
                    self.registers[REG_B] ^= operand as Value;
                }
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo
                // 8 (thereby keeping only its lowest 3 bits), then writes that value to the B
                // register.
                Instruction::Bst => {
                    self.registers[REG_B] = self.combo_value(operand) & 0x7;
                }
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if
                // the A register is not zero, it jumps by setting the instruction pointer to the
                // value of its literal operand; if this instruction jumps, the instruction pointer
                // is not increased by 2 after this instruction.
                Instruction::Jnz => {
                    if self.registers[REG_A] != 0 {
                        assert_eq!(operand % 2, 0);
                        self.program_counter = (operand as usize) / 2;
                        continue;
                    }
                }
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                // register C, then stores the result in register B. (For legacy reasons, this
                // instruction reads an operand but ignores it.)
                Instruction::Bxc => {
                    self.registers[REG_B] ^= self.registers[REG_C];
                }
                // The out instruction (opcode 5) calculates the value of its combo operand modulo
                // 8, then outputs that value. (If a program outputs multiple values, they are
                // separated by commas.)
                Instruction::Out => {
                    let value = self.combo_value(operand) & 0x7;
                    output.push(value as u8);
                }
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that
                // the result is stored in the B register. (The numerator is still read from the A
                // register.)
                Instruction::Bdv => {
                    self.registers[REG_B] = self.registers[REG_A] >> self.combo_value(operand);
                }
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that
                // the result is stored in the C register. (The numerator is still read from the A
                // register.)
                Instruction::Cdv => {
                    self.registers[REG_C] = self.registers[REG_A] >> self.combo_value(operand);
                }
            }
            self.program_counter += 1;
        }

        output
    }

    fn combo_value(&self, operand: u8) -> Value {
        match operand {
            0..=3 => operand as Value,
            4 => self.registers[REG_A],
            5 => self.registers[REG_B],
            6 => self.registers[REG_C],
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const SAMPLE_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    #[test]
    fn test_parse() {
        let computer = ChronospatialComputer::parse(SAMPLE);
        assert_eq!(computer.registers, [729, 0, 0]);
        assert_eq!(
            computer.program,
            vec![
                (Instruction::Adv, 1),
                (Instruction::Out, 4),
                (Instruction::Jnz, 0),
            ]
        );

        println!("{:?}", computer);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&SAMPLE.into()).unwrap(),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE_2.into()).unwrap(), 117440);
    }
}
