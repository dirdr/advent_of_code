use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day7;

impl Solution for Day7 {
    fn part_a(&self, input: &[String]) -> Answer {
        let equations = parse(input);
        equations
            .iter()
            .filter(|e| e.is_true())
            .map(|e| e.left_part)
            .sum::<u64>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

struct Equation {
    left_part: u64,
    right_part: Vec<u64>,
}

impl Equation {
    fn is_true(&self) -> bool {
        fn backtrack(left: u64, right: &Vec<u64>, pos: usize, value: u64) -> bool {
            if pos + 1 == right.len() {
                return value == left;
            }

            let add = backtrack(left, right, pos + 1, value + right[pos + 1]);
            let mul = backtrack(left, right, pos + 1, value * right[pos + 1]);

            add || mul
        }
        backtrack(self.left_part, &self.right_part, 0, self.right_part[0])
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
        assert_eq!(<i32 as Into<Answer>>::into(37), answer);
    }
}
