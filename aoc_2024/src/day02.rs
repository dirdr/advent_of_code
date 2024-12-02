use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day2;

impl Solution for Day2 {
    fn part_a(&self, input: &[String]) -> Answer {
        let reports = parse(input);
        let answer = reports.iter().filter(|r| check_safety(r)).count()
            + reports
                .iter()
                .filter(|r| check_safety(&r.iter().rev().cloned().collect::<Vec<_>>()))
                .count();
        answer.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let reports = parse(input);
        let answer = reports
            .iter()
            .filter(|r| check_safety_tolerant(r, None))
            .count();
        answer.into()
    }
}

fn check_safety(report: &[i32]) -> bool {
    for chunk in report.windows(2) {
        let diff = chunk[1] - chunk[0];
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn check_safety_tolerant(input: &[i32], skip: Option<usize>) -> bool {
    let vals = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| skip.is_none() || Some(*idx) != skip)
        .map(|(_, &x)| x);
    let mut diffs = vals.tuple_windows().map(|(a, b)| a - b).peekable();

    let sig = diffs.peek().unwrap().signum();
    let first_invalid = diffs.position(|x| !(1..=3).contains(&x.abs()) || x.signum() != sig);

    match first_invalid {
        Some(x) if skip.is_none() => {
            check_safety_tolerant(input, Some(x + 1))
                || check_safety_tolerant(input, Some(x.saturating_sub(1)))
                || check_safety_tolerant(input, Some(x))
        }
        None => true,
        _ => false,
    }
}

fn parse(input: &[String]) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day2;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_02_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day2.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_02_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day2.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4), answer);
    }
}
