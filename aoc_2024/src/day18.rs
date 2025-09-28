use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};
use itertools::Itertools;

pub struct Day18;

impl Solution for Day18 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut memory_space = MemorySpace::from_input(input, 71);
        memory_space.simulate_bytes_fall(1024);
        let answer = memory_space.walk();
        answer.len().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Answer::Unimplemented
    }
}

struct MemorySpace {
    map: Matrix<Tile>,
    bytes_fall_predictions: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Corrupted,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Vec2<usize>,
}

impl MemorySpace {
    fn walk(&self) -> Vec<Vec2<usize>> {
        let mut path = vec![];
        let mut pq = BinaryHeap::new();
        pq.push(State {
            cost: 0,
            pos: Vec2::new(0, 0),
        });
        let mut visited = HashSet::new();
        while let Some(State { cost, pos }) = pq.pop() {
            if visited.contains(&(pos, cost)) {
                continue;
            }
            path.push(pos);
            visited.insert((pos, cost));
            println!("{}", pos);
            if pos == Vec2::new(self.map.cols - 1, self.map.rows - 1) {
                return path;
            }
            for dir in Cardinal::all_clockwise() {
                let next = dir.advance(pos.into());
                let Some(&front) = self.map.get(&next) else {
                    continue;
                };
                if front == Tile::Corrupted {
                    continue;
                }

                let next = Vec2::<usize>::try_from(next).unwrap();
                if visited.contains(&(next, cost + 1)) {
                    continue;
                }
                pq.push(State {
                    cost: cost + 1,
                    pos: next,
                });
            }
        }
        unreachable!()
    }

    fn from_input(input: &[String], space_size: usize) -> Self {
        let bytes_fall_predictions = input
            .iter()
            .map(|l| {
                let (x, y) = l.split(',').collect_tuple().unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();
        let mut size = bytes_fall_predictions.iter().max().unwrap();
        Self {
            map: Matrix::new(space_size, space_size, Tile::Empty),
            bytes_fall_predictions,
        }
    }

    fn simulate_bytes_fall(&mut self, amount: usize) {
        for &(x, y) in self.bytes_fall_predictions.iter().take(amount) {
            let pos = Vec2::new(x, y);
            self.map[pos] = Tile::Corrupted;
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Corrupted => f.write_str("#"),
            Self::Empty => f.write_str("."),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use crate::day18::MemorySpace;

    use super::Day18;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_18_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let mut memory_space = MemorySpace::from_input(&input, 7);
        memory_space.simulate_bytes_fall(12);
        let answer = memory_space.walk().len().into();
        assert_eq!(<i32 as Into<Answer>>::into(22), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_18_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day18.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(31), answer);
    }
}
