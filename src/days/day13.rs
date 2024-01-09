use itertools::Itertools;

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day13;

impl Solution for Day13 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed
            .patterns
            .iter()
            .map(|p| {
                Pattern::count_before_reflection(&p.transpose(), 0)
                    + 100 * Pattern::count_before_reflection(&p.rows, 0)
            })
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        parsed
            .patterns
            .iter()
            .map(|p| {
                Pattern::count_before_reflection(&p.transpose(), 1)
                    + 100 * Pattern::count_before_reflection(&p.rows, 1)
            })
            .sum::<usize>()
            .into()
    }
}

struct Parsed {
    patterns: Vec<Pattern>,
}

fn parse(input: &[String]) -> Parsed {
    let mut patterns = vec![];
    let degrouped = input
        .iter()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for group in degrouped {
        let rows = group
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<_>>>();
        patterns.push(Pattern { rows });
    }
    Parsed { patterns }
}

struct Pattern {
    rows: Vec<Vec<char>>,
}

impl Pattern {
    fn transpose(&self) -> Vec<Vec<char>> {
        let rows = self.rows.clone();
        (0..rows[0].len())
            .map(|col| (0..rows.len()).map(|row| rows[row][col]).collect())
            .collect()
    }

    fn count_before_reflection(buffer: &Vec<Vec<char>>, limit: usize) -> usize {
        for mid in 1..=buffer.len() - 1 {
            let side = mid.min(buffer.len() - mid);
            let start = mid - side;
            let mut diff = 0;
            for l in start..mid {
                let r = mid * 2 - l - 1;
                diff += (0..buffer[l].len())
                    .filter(|&i| buffer[l][i] != buffer[r][i])
                    .count();
            }
            if diff == limit {
                return mid;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day13;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_13_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day13.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(405), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_13_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day13.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(400), answer);
    }
}
