use std::collections::{HashMap, HashSet};

use aoc_lib::{answer::Answer, matrix::Matrix, solution::Solution, vec2::Vec2};
use itertools::Itertools;

pub struct Day8;

impl Solution for Day8 {
    fn part_a(&self, input: &[String]) -> Answer {
        let map = Map::from_input(input);
        map.count_valid_antinodes(false).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let map = Map::from_input(input);
        map.count_valid_antinodes(true).into()
    }
}

struct Map {
    map: Matrix<char>,
    antennas: HashMap<char, Vec<Vec2<usize>>>,
}

impl Map {
    fn from_input(input: &[String]) -> Self {
        let map = Matrix::from_chars(input);
        let mut antennas: HashMap<char, Vec<Vec2<usize>>> = HashMap::new();
        for y in 0..map.rows {
            for x in 0..map.cols {
                let pos = Vec2::new(x, y);
                if map[pos] == '.' {
                    continue;
                }
                antennas.entry(map[pos]).or_default().push(pos);
            }
        }
        Self { map, antennas }
    }

    fn count_valid_antinodes(&self, count_harmonic: bool) -> usize {
        let mut uniques: HashSet<Vec2<usize>> = HashSet::new();
        for positions in self.antennas.values() {
            for perm in positions.iter().permutations(2) {
                let to = Vec2::<isize>::from(perm[0]);
                let from = Vec2::<isize>::from(perm[1]);

                let positions = if count_harmonic {
                    (0..)
                        .map(|k| to + (to - from) * k)
                        .take_while(|pos| self.map.contains(pos))
                        .collect::<Vec<_>>()
                } else {
                    let pos = to + (to - from);
                    let mut out = vec![];
                    if self.map.contains(&pos) {
                        out.push(pos);
                    }
                    out
                };

                uniques.extend(
                    positions
                        .iter()
                        .map(|p| Vec2::<usize>::try_from(p).unwrap()),
                );
            }
        }
        uniques.len()
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day8;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_08_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day8.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(14), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_08_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day8.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(34), answer);
    }
}
