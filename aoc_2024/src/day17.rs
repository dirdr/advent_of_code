use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day17;

impl Solution for Day17 {
    fn part_a(&self, input: &[String]) -> Answer {
        Computer::from_input(input).run_program().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Answer::Unimplemented
    }
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
    registers: [u32; 3],
    instructions: Vec<Instruction>,
}

impl Computer {
    fn from_input(input: &[String]) -> Self {
        let mut instructions = vec![];
        let (regs, program) = input.split(|l| l.is_empty()).collect_tuple().unwrap();
        let reg_a = regs[0][12..].parse::<u32>().unwrap();
        let reg_b = regs[1][12..].parse::<u32>().unwrap();
        let reg_c = regs[2][12..].parse::<u32>().unwrap();

        let (_, program) = program[0].split_once(' ').unwrap();

        for token in program
            .split(',')
            .map(|e| e.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
        {
            instructions.push(Instruction::from_consecutive_tokens(token[0], token[1]))
        }
        Self {
            registers: [reg_a, reg_b, reg_c],
            instructions,
        }
    }

    fn run_program(mut self) -> String {
        let mut out = vec![];
        let mut instruction_pointer = 0;

        while instruction_pointer < self.instructions.len() {
            let instruction = &self.instructions[instruction_pointer];
            if let Instruction::Jnz(_) = instruction {
                if let Some(new_pointer) = instruction.run_side_effect(&self.registers) {
                    instruction_pointer = new_pointer;
                } else {
                    instruction_pointer += 1;
                }
            } else {
                instruction.run(&mut self.registers, &mut out);
                instruction_pointer += 1;
            }
        }

        out.iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(",")
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

    fn run(&self, registers: &mut [u32; 3], out: &mut Vec<u8>) {
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

    fn run_side_effect(&self, registers: &[u32; 3]) -> Option<usize> {
        if let Self::Jnz(o) = self {
            return Self::jnz(registers, *o);
        }
        unreachable!()
    }

    fn adv(registers: &mut [u32; 3], operand: u8) {
        let num = registers[0];
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {}", other);
                return;
            }
        };
        let result = num / denominator;
        registers[0] = result;
    }

    fn bxl(registers: &mut [u32; 3], operand: u8) {
        registers[1] ^= operand as u32;
    }

    fn bst(registers: &mut [u32; 3], operand: u8) {
        registers[1] = match operand {
            n @ 0..=3 => n as u32,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            other => {
                println!("Unsupported operand: {}", other);
                return;
            }
        } % 8;
    }

    fn jnz(registers: &[u32; 3], operand: u8) -> Option<usize> {
        if registers[0] != 0 {
            return Some(operand as usize);
        }
        None
    }

    fn bxc(registers: &mut [u32; 3]) {
        registers[1] ^= registers[2];
    }

    fn out(output: &mut Vec<u8>, registers: &[u32; 3], operand: u8) {
        let token: u32 = match operand {
            n @ 0..=3 => n as u32,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            other => {
                println!("Unsupported operand: {}", other);
                return;
            }
        };
        output.push((token % 8) as u8);
    }

    fn bdv(registers: &mut [u32; 3], operand: u8) {
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {}", other);
                return;
            }
        };
        let result = registers[0] / denominator;
        registers[1] = result;
    }

    fn cdv(registers: &mut [u32; 3], operand: u8) {
        let denominator = match operand {
            n @ 0..=3 => 1 << n,
            4 => 1 << registers[0],
            5 => 1 << registers[1],
            6 => 1 << registers[2],
            other => {
                println!("Unsupported operand: {}", other);
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
            input::read_file(&format!("{}day_17_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day17.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(11), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_17_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day17.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(31), answer);
    }
}
