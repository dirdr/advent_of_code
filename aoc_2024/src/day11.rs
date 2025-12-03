use std::collections::HashMap;

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day11;

impl Solution for Day11 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut stones = Stones::from_input(input);
        solve(&mut stones, 25).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut stones = Stones::from_input(input);
        solve(&mut stones, 75).into()
    }
}

fn solve(stones: &mut Stones, iterations: usize) -> u64 {
    for _ in 0..iterations {
        stones.blink();
    }
    stones.stones_count.values().sum::<u64>()
}

#[derive(Debug)]
struct Stones {
    stones_count: HashMap<u64, u64>,
}

impl Stones {
    fn from_input(input: &[String]) -> Self {
        let stones = input[0]
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut stones_count = HashMap::new();
        stones
            .into_iter()
            .for_each(|s| *stones_count.entry(s).or_default() += 1);
        Self { stones_count }
    }

    fn blink(&mut self) {
        // To avoid mutating the map why we iter it
        let mut next: HashMap<u64, u64> = HashMap::new();
        for (&stone, &count) in self.stones_count.iter() {
            if stone == 0 {
                *next.entry(1).or_default() += count
            } else if let Some((left, right)) = split_stone(stone) {
                *next.entry(left).or_default() += count;
                *next.entry(right).or_default() += count
            } else {
                *next.entry(stone * 2024).or_default() += count
            }
        }
        std::mem::swap(&mut self.stones_count, &mut next);
    }
}

fn split_stone(stone: u64) -> Option<(u64, u64)> {
    let num_digit = (stone as f64).log10() as u32 + 1;
    if !num_digit.is_multiple_of(2) {
        return None;
    }
    let temp = 10u64.pow(num_digit / 2);
    let left = stone / temp;
    let right = stone % temp;
    Some((left, right))
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day11;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_11_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day11.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(55312), answer);
    }
}
