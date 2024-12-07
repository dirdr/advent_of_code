use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day7;

impl Solution for Day7 {
    fn part_a(&self, input: &[String]) -> Answer {
        let equations = parse(input);
        solve(equations, &[Operation::Add, Operation::Mul]).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let equations = parse(input);
        solve(
            equations,
            &[Operation::Add, Operation::Mul, Operation::Concat],
        )
        .into()
    }
}

fn solve(equations: Vec<Equation>, operations: &[Operation]) -> u64 {
    equations
        .iter()
        .filter(|e| e.is_true(operations))
        .map(|e| e.left_part)
        .sum::<u64>()
}

struct Equation {
    left_part: u64,
    right_part: Vec<u64>,
}

enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Mul => a * b,
            Operation::Concat => a * 10_u64.pow((b as f64).log10() as u32 + 1) + b,
        }
    }
}

impl Equation {
    fn is_true(&self, ops: &[Operation]) -> bool {
        fn backtrack(
            left: u64,
            right: &Vec<u64>,
            pos: usize,
            value: u64,
            ops: &[Operation],
        ) -> bool {
            if pos + 1 == right.len() {
                return value == left;
            }
            ops.iter()
                .any(|o| backtrack(left, right, pos + 1, o.evaluate(value, right[pos + 1]), ops))
        }
        backtrack(self.left_part, &self.right_part, 0, self.right_part[0], ops)
    }
}

fn parse(input: &[String]) -> Vec<Equation> {
    let mut equations = vec![];
    for line in input {
        let (left, right) = line.split_once(":").unwrap();
        let left_part = left.parse::<u64>().unwrap();
        let right_part = right
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        equations.push(Equation {
            left_part,
            right_part,
        })
    }
    equations
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day7;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_07_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day7.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(3749), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_07_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day7.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(11387), answer);
    }
}
