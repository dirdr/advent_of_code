use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, input: &[String]) -> Answer {
        let problem = Problem::from_input(input);
        problem
            .banks
            .iter()
            .map(|bank| bank.maximum_joltage(2))
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let problem = Problem::from_input(input);
        problem
            .banks
            .iter()
            .map(|bank| bank.maximum_joltage(12))
            .sum::<usize>()
            .into()
    }
}

impl Problem {
    fn from_input(input: &[String]) -> Self {
        let banks = input
            .iter()
            .map(|e| Bank {
                batteries: e
                    .chars()
                    .map(|d| d.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();
        Self { banks }
    }
}

impl Bank {
    fn maximum_joltage(&self, allowed: usize) -> usize {
        let mut stack = vec![];
        let remove_allowed = self.batteries.len() - allowed;
        let mut to_remove = remove_allowed;

        for &b in &self.batteries {
            while to_remove > 0 && !stack.is_empty() && b > *stack.last().unwrap() {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(b);
        }

        let mut num = 0;
        for (i, el) in stack.iter().enumerate().take(allowed) {
            num += 10usize.pow((allowed - i - 1) as u32) * *el as usize;
        }
        num
    }
}

struct Problem {
    banks: Vec<Bank>,
}

struct Bank {
    batteries: Vec<u8>,
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day3;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_03_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day3.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(357), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_03_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day3.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(3121910778619), answer);
    }
}
