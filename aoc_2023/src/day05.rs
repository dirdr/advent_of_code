use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

pub struct Day5;

impl Solution for Day5 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let seeds = parsed.seeds;
        let mut min = usize::MAX;
        for mut seed in seeds {
            for map in &parsed.maps {
                seed = map.map(seed);
            }
            min = min.min(seed);
        }
        min.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let seeds = parsed.seeds;
        seeds
            .par_chunks_exact(2)
            .map(|sr| {
                let mut min = usize::MAX;
                for mut seed in sr[0]..=(sr[0] + sr[1]) {
                    for map in &parsed.maps {
                        seed = map.map(seed);
                    }
                    min = min.min(seed);
                }
                min
            })
            .min()
            .unwrap()
            .into()
    }
}

struct Parsed {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

struct Map {
    ranges: Vec<Range>,
}

struct Range {
    src: usize,
    dest: usize,
    len: usize,
}

fn parse(input: &[String]) -> Parsed {
    let mut lines = input.iter();
    let (_, seeds) = lines.next().unwrap().split_once("seeds: ").unwrap();
    let seeds: Vec<usize> = seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let degrouped = lines
        .chunk_by(|line| !line.is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut maps = vec![];
    for m in degrouped {
        let mut ranges = vec![];
        for entry in m {
            let entry: Vec<usize> = entry
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let (dest, src, len) = (entry[0], entry[1], entry[2]);
            ranges.push(Range { src, dest, len })
        }
        maps.push(Map { ranges });
    }
    Parsed { seeds, maps }
}

impl Map {
    fn map(&self, seed: usize) -> usize {
        for range in &self.ranges {
            if range.contains(seed) {
                return range.dest + seed - range.src;
            }
        }
        seed
    }
}

impl Range {
    fn contains(&self, value: usize) -> bool {
        value >= self.src && value < (self.src + self.len)
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day5;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(35), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_05_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day5.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(46), answer);
    }
}
