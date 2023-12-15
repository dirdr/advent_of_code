use std::collections::HashSet;

use crate::helper_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day5;

impl Solution for Day5 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut lines = input.iter();
        let (_, seeds) = lines.next().unwrap().split_once("seeds: ").unwrap();
        let mut mapped_seeds: Vec<usize> = seeds
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let degrouped = lines
            .into_iter()
            .group_by(|line| !line.is_empty())
            .into_iter()
            .filter(|(key, _)| *key)
            .map(|(_, group)| group.skip(1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for map in degrouped {
            let mut processed_seeds: HashSet<usize> = HashSet::new();
            for entry in map {
                let entry: Vec<usize> = entry
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let (dest, source, len) = (entry[0], entry[1], entry[2]);
                for val in mapped_seeds.iter_mut() {
                    if *val >= source && *val < (source + len) && !processed_seeds.contains(&val) {
                        let delta = *val - source;
                        *val = dest + delta;
                        processed_seeds.insert(*val);
                    }
                }
            }
            processed_seeds.clear();
        }
        let min = *mapped_seeds.iter().min().unwrap();
        min.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        0.into()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day5;

    #[test]
    pub fn test() {
        let input =
            input::read_file(&format!("{}day_5_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day5.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(35i32), answer);
    }
}
