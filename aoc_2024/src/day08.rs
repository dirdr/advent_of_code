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
        for &node_type in self.antennas.keys() {
            let positions = self.antennas[&node_type].clone();
            for perm in positions.iter().permutations(2) {
                let first_pos = Vec2::<isize>::from(perm[0]);
                let second_pos = Vec2::<isize>::from(perm[1]);
                let mut anti_nodes = vec![];

                if count_harmonic {
                    anti_nodes.push(first_pos);
                    let mut k = 0;
                    while self
                        .map
                        .contains(&((first_pos) + (first_pos - second_pos) * k))
                    {
                        anti_nodes.push((first_pos) + (first_pos - second_pos) * k);
                        k += 1;
                    }
                } else {
                    let pos = (first_pos * 2) - second_pos;
                    if self.map.contains(&pos) {
                        anti_nodes.push(pos);
                    }
                }
                for an in anti_nodes {
                    uniques.insert(Vec2::<usize>::try_from(an).unwrap());
                }
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
