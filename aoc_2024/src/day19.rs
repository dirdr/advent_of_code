use std::collections::HashMap;

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day19;

impl Solution for Day19 {
    fn part_a(&self, input: &[String]) -> Answer {
        let pb = Problem::from_input(input);
        pb.count_ways_of_creating_patterns()
            .iter()
            .filter(|&&w| w > 0)
            .count()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let pb = Problem::from_input(input);
        pb.count_ways_of_creating_patterns()
            .iter()
            .sum::<usize>()
            .into()
    }
}

struct Problem {
    towels: Vec<Vec<Color>>,
    patterns: Vec<Vec<Color>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    W,
    U,
    B,
    R,
    G,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Color::W,
            'u' => Color::U,
            'b' => Color::B,
            'r' => Color::R,
            'g' => Color::G,
            _ => unreachable!(),
        }
    }
}

impl Problem {
    fn from_input(input: &[String]) -> Self {
        let mut parts = input.splitn(2, |line| line.trim().is_empty());
        let towels = parts.next().unwrap()[0]
            .split(',')
            .map(|part| part.trim().chars().map(Color::from).collect::<_>())
            .collect::<Vec<_>>();

        let patterns = parts
            .next()
            .unwrap()
            .iter()
            .map(|s| s.trim().chars().map(Color::from).collect::<_>())
            .collect::<Vec<_>>();

        Self { towels, patterns }
    }

    fn count_ways_of_creating_patterns(&self) -> Vec<usize> {
        let mut answer = vec![];
        let mut memo = HashMap::new();
        for pattern in &self.patterns {
            answer.push(Self::backtrack(pattern, &self.towels, &mut memo));
        }
        answer
    }

    fn backtrack(
        pattern: &[Color],
        availables: &[Vec<Color>],
        memo: &mut HashMap<Vec<Color>, usize>,
    ) -> usize {
        if let Some(&possible) = memo.get(&pattern.to_vec()) {
            return possible;
        }

        if pattern.is_empty() {
            return 1;
        }

        let mut count = 0;
        for a in availables {
            if pattern.starts_with(a) {
                let child = Self::backtrack(&pattern[a.len()..], availables, memo);
                count += child;
            }
        }
        memo.insert(pattern.to_vec(), count);
        count
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day19;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_19_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day19.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_19_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day19.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(16), answer);
    }
}
