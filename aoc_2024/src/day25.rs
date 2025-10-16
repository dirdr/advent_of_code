use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day25;

impl Solution for Day25 {
    fn part_a(&self, input: &[String]) -> Answer {
        let problem = Problem::from_input(input);
        let answer = problem
            .keys
            .iter()
            .flat_map(|key| problem.locks.iter().filter(move |lock| !overlap(key, lock)))
            .count();

        answer.into()
    }

    fn part_b(&self, _: &[String]) -> Answer {
        Answer::Unimplemented
    }
}

impl Problem {
    fn from_input(input: &[String]) -> Self {
        let (keys, locks): (Vec<_>, Vec<_>) = input
            .split(|line| line.is_empty())
            .map(|block| {
                let grid: Vec<Vec<char>> = block.iter().map(|l| l.chars().collect()).collect();
                let is_lock = grid
                    .first()
                    .is_some_and(|row| row.iter().all(|&c| c == '#'))
                    && grid.last().is_some_and(|row| row.iter().all(|&c| c == '.'));

                let heights = (0..grid[0].len())
                    .map(|c| (1..grid.len() - 1).filter(|&r| grid[r][c] == '#').count())
                    .collect::<Vec<_>>();

                if is_lock {
                    Either::Lock(Lock { heights })
                } else {
                    let reversed = (0..grid[0].len())
                        .map(|c| {
                            (1..grid.len() - 1)
                                .rev()
                                .filter(|&r| grid[r][c] == '#')
                                .count()
                        })
                        .collect::<Vec<_>>();
                    Either::Key(Key { heights: reversed })
                }
            })
            .partition_map(|e| match e {
                Either::Key(k) => itertools::Either::Left(k),
                Either::Lock(l) => itertools::Either::Right(l),
            });

        Self { keys, locks }
    }
}

fn overlap(key: &Key, lock: &Lock) -> bool {
    key.heights
        .iter()
        .zip(&lock.heights)
        .any(|(k, l)| k + l > 5)
}

struct Problem {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

struct Key {
    heights: Vec<usize>,
}

struct Lock {
    heights: Vec<usize>,
}

enum Either {
    Key(Key),
    Lock(Lock),
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::{input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_25_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day25.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(3), answer);
    }
}
