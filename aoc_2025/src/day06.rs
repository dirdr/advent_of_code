use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day6;

impl Solution for Day6 {
    fn part_a(&self, input: &[String]) -> Answer {
        ProblemSheet::from_input_a(input).solve().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        ProblemSheet::from_input_b(input).solve().into()
    }
}

impl ProblemSheet {
    fn solve(&self) -> usize {
        self.problems.iter().map(|p| p.solve()).sum::<usize>()
    }

    fn from_input_a(input: &[String]) -> Self {
        let r = input.len();
        let lines: Vec<Vec<usize>> = input[..r - 1]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|d| d.parse().unwrap())
                    .collect()
            })
            .collect();

        let operations: Vec<Operation> = input[r - 1]
            .split_whitespace()
            .map(Operation::from)
            .collect();

        let problems = (0..lines[0].len())
            .map(|c| {
                let column = lines.iter().map(|row| row[c]).collect();
                Problem {
                    column,
                    operation: operations[c],
                }
            })
            .collect();

        Self { problems }
    }

    fn from_input_b(input: &[String]) -> Self {
        let parse_number = |num: &[char]| -> usize {
            let mut curr = 0;
            let mut mul = 1;
            for ch in num.iter().rev() {
                if !ch.is_ascii_digit() {
                    continue;
                }
                curr += ch.to_digit(10).unwrap() * mul;
                mul *= 10;
            }
            curr as usize
        };

        let input = input
            .iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let (r, c) = (input.len(), input[0].len());

        let mut columns = vec![];
        for col in (0..c).rev() {
            let mut curr = vec![];
            for row in input.iter().take(r) {
                curr.push(row[col]);
            }
            columns.push(curr.clone());
        }

        let mut problems = vec![];
        let groups = columns
            .split(|line| line.iter().all(|&e| e == ' '))
            .collect::<Vec<_>>();

        for group in groups {
            let operation = Operation::from(*group.last().unwrap().last().unwrap());
            let column = group
                .iter()
                .map(|l| parse_number(&l[..]))
                .collect::<Vec<_>>();
            problems.push(Problem { operation, column })
        }

        ProblemSheet { problems }
    }
}

struct ProblemSheet {
    problems: Vec<Problem>,
}

struct Problem {
    column: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        let mut answer = self.column[0];
        for &num in self.column.iter().skip(1) {
            answer = self.operation.apply(answer, num);
        }
        answer
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '*' => Operation::Multiply,
            '+' => Operation::Add,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day6;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_06_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day6.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4277556), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_06_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day6.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(3263827), answer);
    }
}
