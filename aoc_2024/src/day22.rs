use aoc_lib::{answer::Answer, solution::Solution};
use std::collections::HashMap;

pub struct Day22;

impl Solution for Day22 {
    fn part_a(&self, input: &[String]) -> Answer {
        let seeds = parse(input);
        seeds
            .iter()
            .map(|&seed| nth_secret(seed, 2000))
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let seeds = parse(input);
        find_best_sequence(&seeds, 2000).into()
    }
}

fn find_best_sequence(seeds: &[usize], iterations: usize) -> usize {
    let mut sequence_totals: HashMap<[i8; 4], usize> = HashMap::new();

    for &seed in seeds {
        let mut seen = HashMap::new();

        let mut secret = seed;
        let mut prev_price = (seed % 10) as i8;
        let mut window = Vec::new();

        for _ in 0..iterations {
            secret = next_secret(secret);
            let price = (secret % 10) as i8;
            let delta = price - prev_price;

            window.push(delta);
            if window.len() > 4 {
                window.remove(0);
            }

            if window.len() == 4 {
                let seq: [i8; 4] = window.clone().try_into().unwrap();
                if let std::collections::hash_map::Entry::Vacant(e) = seen.entry(seq) {
                    e.insert(price as usize);
                    *sequence_totals.entry(seq).or_insert(0) += price as usize;
                }
            }

            prev_price = price;
        }
    }

    sequence_totals.values().max().copied().unwrap_or(0)
}

fn nth_secret(seed: usize, n: usize) -> usize {
    let mut secret = seed;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

fn next_secret(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}

fn prune(secret: usize) -> usize {
    secret & 0xFFFFFF
}

fn parse(input: &[String]) -> Vec<usize> {
    input.iter().filter_map(|line| line.parse().ok()).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_lib::{answer::Answer, input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_22_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day22.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(37327623), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_22_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day22.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(23), answer);
    }
}
