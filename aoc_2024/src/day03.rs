use aoc_lib::{answer::Answer, solution::Solution};
use regex::Regex;

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut answer = 0;
        for line in input {
            let instructions = extract_instructions(line.as_str());
            for inst in instructions {
                if let Instruction::Mul(first, second) = inst {
                    answer += first * second;
                }
            }
        }
        answer.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut answer = 0;
        let mut enable = true;
        for line in input {
            let instructions = extract_instructions(line.as_str());
            for inst in instructions {
                match inst {
                    Instruction::Do => enable = true,
                    Instruction::Dont => enable = false,
                    Instruction::Mul(first, second) if enable => answer += first * second,
                    Instruction::Mul(_, _) => (),
                }
            }
        }
        answer.into()
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn extract_instructions(s: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(s)
        .filter_map(|cap| match cap.get(0).map(|m| m.as_str()) {
            Some(full_match) if full_match.starts_with("mul(") => cap[2]
                .parse::<i32>()
                .ok()
                .zip(cap[3].parse::<i32>().ok())
                .map(|(x, y)| Instruction::Mul(x, y)),
            Some("do()") => Some(Instruction::Do),
            Some("don't()") => Some(Instruction::Dont),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day3;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_03_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day3.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(161), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_03_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day3.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(48), answer);
    }
}
