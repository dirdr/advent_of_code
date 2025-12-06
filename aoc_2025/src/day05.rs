use std::ops::RangeInclusive;

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day5;

impl Solution for Day5 {
    fn part_a(&self, input: &[String]) -> Answer {
        Database::from_input(input).part_a().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Database::from_input(input).part_b().into()
    }
}

impl Database {
    fn part_a(&self) -> usize {
        let mut total = 0;
        for &ingredient in &self.ingredients {
            for range in &self.ranges {
                if range.contains(&ingredient) {
                    total += 1;
                    break;
                }
            }
        }
        total
    }

    fn part_b(&self) -> usize {
        let merged = self.merge_intervals();
        merged
            .iter()
            .fold(0, |acc, x| acc + x.end() - x.start() + 1)
    }

    fn merge_intervals(&self) -> Vec<RangeInclusive<usize>> {
        let mut sorted = self.ranges.clone();
        sorted.sort_unstable_by(|a, b| a.start().cmp(b.start()));
        let mut merged = vec![sorted[0].clone()];
        for curr in sorted.iter().skip(1) {
            let last = merged.last().unwrap();
            if curr.start() <= last.end() {
                let mut end = *last.start()..=*(last.end().max(curr.end()));
                std::mem::swap(merged.last_mut().unwrap(), &mut end);
            } else {
                merged.push(curr.clone());
            }
        }
        merged
    }

    fn from_input(input: &[String]) -> Self {
        let mut input = input.split(|l| l.is_empty());
        let ranges = input
            .next()
            .expect("ranges are expected before the blank line");
        let ingredients = input
            .next()
            .expect("ingredients are expected after blank line");

        let ranges = ranges
            .iter()
            .map(|l| {
                let (a, b) = l.as_str().split_once('-').unwrap();
                let (a, b) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
                a..=b
            })
            .collect::<Vec<_>>();
        let ingredients = ingredients
            .iter()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            ranges,
            ingredients,
        }
    }
}

struct Database {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day5;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(3), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(14), answer);
    }
}
