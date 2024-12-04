use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day9;

impl Solution for Day9 {
    fn part_a(&self, input: &[String]) -> Answer {
        let reports = parse(input);
        reports.iter().fold(0, |acc, x| acc + x.predict_a()).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let reports = parse(input);
        reports.iter().fold(0, |acc, x| acc + x.predict_b()).into()
    }
}

fn parse(input: &[String]) -> Vec<Report> {
    let mut reports = vec![];
    for line in input {
        let history = line
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();
        reports.push(Report { history });
    }
    reports
}

struct Report {
    history: Vec<i32>,
}

impl Report {
    fn reduce(&self) -> Vec<Vec<i32>> {
        let mut sequences = vec![self.history.clone()];
        loop {
            let top: Vec<i32> = sequences.last().cloned().unwrap();
            if top.iter().all(|&e| e == 0) {
                break;
            }
            let new_sequence = top.windows(2).map(|window| window[1] - window[0]).collect();
            sequences.push(new_sequence);
        }
        sequences
    }

    fn predict_a(&self) -> i32 {
        let mut sequences = self.reduce();
        sequences.last_mut().unwrap().push(0);
        while let Some(top_sequence) = sequences.pop() {
            if let Some(last_sequence) = sequences.last_mut() {
                if let (Some(&last_value), Some(&top_value)) =
                    (last_sequence.last(), top_sequence.last())
                {
                    last_sequence.push(last_value + top_value);
                } else {
                    return 0;
                }
            }
            if sequences.len() == 1 {
                return sequences[0].last().copied().unwrap_or(0);
            }
        }
        0
    }

    fn predict_b(&self) -> i32 {
        let mut sequences = self.reduce();
        sequences.last_mut().unwrap().insert(0, 0);
        while let Some(top_sequences) = sequences.pop() {
            if let Some(last_sequence) = sequences.last_mut() {
                if let (Some(&value), Some(&top_value)) =
                    (last_sequence.first(), top_sequences.first())
                {
                    last_sequence.insert(0, value - top_value);
                } else {
                    return 0;
                }
            }
            if sequences.len() == 1 {
                return sequences[0].first().copied().unwrap_or(0);
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day9;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day9.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(114), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day9.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2), answer);
    }
}
