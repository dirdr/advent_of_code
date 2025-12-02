use std::{collections::HashSet, vec};

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day2;

impl Solution for Day2 {
    fn part_a(&self, input: &[String]) -> Answer {
        let problem = Problem::parse(input);
        problem.find_invalid_ids(SearchMode::Twice).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let problem = Problem::parse(input);
        problem.find_invalid_ids(SearchMode::All).into()
    }
}

impl Problem {
    fn parse(input: &[String]) -> Self {
        let mut ranges = vec![];
        for line in input.first().unwrap().split(',') {
            let (l, r) = line.split_once('-').unwrap();
            let range = Range {
                start: l.parse::<usize>().unwrap(),
                end: r.parse::<usize>().unwrap(),
            };
            ranges.push(range);
        }
        Self { ranges }
    }

    fn find_invalid_ids(&self, search_mode: SearchMode) -> usize {
        let mut set = HashSet::new();
        for range in &self.ranges {
            let ids = range.find_invalid_ids(&search_mode);
            for &id in &ids {
                set.insert(id);
            }
        }
        set.iter().sum::<usize>()
    }
}

impl Range {
    fn find_invalid_ids(&self, search_mode: &SearchMode) -> Vec<usize> {
        let mut invalids = vec![];
        for num in self.start..=self.end {
            let owned = num.to_string();
            match search_mode {
                SearchMode::Twice => {
                    let str = owned.as_str();
                    if str[..(str.len() / 2)] == str[(str.len() / 2)..] {
                        invalids.push(num);
                    }
                }
                SearchMode::All => {
                    for size in 1..=(owned.len().div_ceil(2)) {
                        let vectorized = owned.chars().collect::<Vec<_>>();
                        let mut chunks = vectorized.chunks(size);
                        if chunks.len() < 2 {
                            continue;
                        }
                        if let Some(first) = chunks.next()
                            && chunks.all(|e| e == first)
                        {
                            invalids.push(num);
                        }
                    }
                }
            }
        }
        invalids
    }
}

enum SearchMode {
    Twice,
    All,
}

struct Problem {
    ranges: Vec<Range>,
}

struct Range {
    start: usize,
    end: usize,
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
        assert_eq!(<i32 as Into<Answer>>::into(1227775554), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_02_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day2.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(4174379265), answer);
    }
}
