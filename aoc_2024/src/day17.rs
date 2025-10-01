use std::collections::VecDeque;

use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day17;

impl Solution for Day17 {
    fn part_a(&self, input: &[String]) -> Answer {
        let (computer, mut registers) = parse_input(input);
        let out = computer.run_program(&mut registers);

        out.iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let (computer, _) = parse_input(input);
        let program_len = computer.raw_program.len();
        let mut to_visit = VecDeque::from([(program_len, 0)]);

        while let Some((pos, a)) = to_visit.pop_front() {
            for i in 0..0b111u64 {
                let n_a = (a << 3) | i;
                let out = computer.run_program(&mut [n_a, 0, 0]);
                if out == computer.raw_program[pos - 1..] {
                    to_visit.push_back((pos - 1, n_a));
                    if out.len() == program_len {
                        return n_a.into();
                    }
                }
            }
        }
        Answer::Unimplemented
    }
}

fn parse_input(input: &[String]) -> (Computer, [u64; 3]) {
    let mut instructions = vec![];
    let (regs, program) = input.split(|l| l.is_empty()).collect_tuple().unwrap();
    let reg_a = regs[0][12..].parse::<u64>().unwrap();
    let reg_b = regs[1][12..].parse::<u64>().unwrap();
    let reg_c = regs[2][12..].parse::<u64>().unwrap();

    let (_, program) = program[0].split_once(' ').unwrap();

    let raw_program = program
        .split(',')
        .map(|e| e.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    for token in raw_program.chunks(2) {
        instructions.push(Instruction::from_consecutive_tokens(token[0], token[1]))
    }
    (
        Computer {
            instructions,
            raw_program,
        },
        [reg_a, reg_b, reg_c],
    )
}

enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

struct Computer {
    instructions: Vec<Instruction>,
    raw_program: Vec<u8>,
}

impl Computer {
    fn run_program(&self, registers: &mut [u64; 3]) -> Vec<u8> {
        let mut out = vec![];
        let mut instruction_pointer = 0;

        while instruction_pointer < self.instructions.len() {
            let instruction = &self.instructions[instruction_pointer];
            if let Instruction::Jnz(_) = instruction {
                if let Some(new_pointer) = instruction.run_side_effect(registers) {
                    instruction_pointer = new_pointer;
                } else {
                    instruction_pointer += 1;
                }
            } else {
                instruction.run(registers, &mut out);
                instruction_pointer += 1;
            }
        }
        out
    }
}

impl Instruction {
    fn from_consecutive_tokens(a: u8, b: u8) -> Self {
        match a {
            0 => Self::Adv(b),
            1 => Self::Bxl(b),
            2 => Self::Bst(b),
            3 => Self::Jnz(b),
            4 => Self::Bxc,
            5 => Self::Out(b),
            6 => Self::Bdv(b),
            7 => Self::Cdv(b),
            _ => unreachable!(),
        }
    }

    fn run(&self, registers: &mut [u64; 3], out: &mut Vec<u8>) {
        match self {
            Instruction::Adv(o) => Self::adv(registers, *o),
            Instruction::Bxl(o) => Self::bxl(registers, *o),
            Instruction::Bst(o) => Self::bst(registers, *o),
            Instruction::Bxc => Self::bxc(registers),
            Instruction::Out(o) => Self::out(out, registers, *o),
            Instruction::Bdv(o) => Self::bdv(registers, *o),
            Instruction::Cdv(o) => Self::cdv(registers, *o),
            _ => unreachable!(),
        }
    }

    fn run_side_effect(&self, registers: &[u64; 3]) -> Option<usize> {
        if let Self::Jnz(o) = self {
            return Self::jnz(registers, *o);
        }
        unreachable!()
    }

    fn adv(registers: &mut [u64; 3], operand: u8) {
        let num = registers[0];
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {other}");
                return;
            }
        };
        let result = num / denominator;
        registers[0] = result;
    }

    fn bxl(registers: &mut [u64; 3], operand: u8) {
        registers[1] ^= operand as u64;
    }

    fn bst(registers: &mut [u64; 3], operand: u8) {
        registers[1] = match operand {
            n @ 0..=3 => n as u64,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            other => {
                println!("Unsupported operand: {other}");
                return;
            }
        } % 8;
    }

    fn jnz(registers: &[u64; 3], operand: u8) -> Option<usize> {
        if registers[0] != 0 {
            return Some(operand as usize);
        }
        None
    }

    fn bxc(registers: &mut [u64; 3]) {
        registers[1] ^= registers[2];
    }

    fn out(output: &mut Vec<u8>, registers: &[u64; 3], operand: u8) {
        let token: u64 = match operand {
            n @ 0..=3 => n as u64,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            other => {
                println!("Unsupported operand: {other}");
                return;
            }
        };
        output.push((token % 8) as u8);
    }

    fn bdv(registers: &mut [u64; 3], operand: u8) {
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {other}");
                return;
            }
        };
        let result = registers[0] / denominator;
        registers[1] = result;
    }

    fn cdv(registers: &mut [u64; 3], operand: u8) {
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {other}");
                return;
            }
        };
        let result = registers[0] / denominator;
        registers[2] = result;
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day17;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_17_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day17.part_a(&input);
        assert_eq!(
            <String as Into<Answer>>::into("4,6,3,5,6,3,5,2,1,0".to_string()),
            answer
        );
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_17_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day17.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(117440), answer);
    }
}
