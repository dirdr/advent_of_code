use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day9;

struct Parsed {
    reports: Vec<Report>,
}

struct Report {
    history: Vec<i32>,
}

fn parse(input: &[String]) -> Parsed {
    let mut reports = vec![];
    for line in input {
        reports.push(Report {
            history: line
                .split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect(),
        });
    }
    Parsed { reports }
}

impl Solution for Day9 {
    fn part_a(&self, input: &[String]) -> Answer {
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day9;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_9_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day9.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2i32), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_9_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day9.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6i32), answer);
    }
}
