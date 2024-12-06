use std::collections::HashSet;

use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day6;

impl Solution for Day6 {
    fn part_a(&self, input: &[String]) -> Answer {
        let map = parse(input);
        map.walk().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let map = parse(input);
        let mut count = 0;
        for y in 0..map.grid.rows {
            for x in 0..map.grid.cols {
                let pos = Vec2::new(x, y);
                if map.grid[pos] == '#' || map.grid[pos] == '^' {
                    continue;
                }
                let mut modified = map.grid.clone();
                modified[pos] = '#';
                let new = Map {
                    grid: modified,
                    starting_pos: map.starting_pos,
                };
                if new.walk() == -1 {
                    count += 1;
                }
            }
        }
        count.into()
    }
}

fn parse(input: &[String]) -> Map {
    let grid = Matrix::from_chars(input);
    let starting_pos = grid.find('^').unwrap();
    Map { grid, starting_pos }
}

struct Map {
    grid: Matrix<char>,
    starting_pos: Vec2<usize>,
}

impl Map {
    fn walk(&self) -> isize {
        let max_loop_threshold = self.grid.rows * self.grid.cols;
        let mut pos = Vec2::<isize>::from(self.starting_pos);
        let mut dir = Cardinal::North;
        let mut visited: HashSet<Vec2<usize>> = HashSet::new();
        let mut count = 0;
        loop {
            if count > max_loop_threshold {
                return -1;
            }
            if visited.insert(Vec2::<usize>::try_from(pos).unwrap()) {
                count = 0;
            } else {
                count += 1;
            }
            let next = dir.advance(pos);
            let Some(&ch) = self.grid.get(&next) else {
                break;
            };
            if ch == '#' {
                dir = dir.turn_right();
            } else {
                pos = next;
            }
        }
        visited.len() as isize
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day6;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_06_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day6.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(41), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_06_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day6.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6), answer);
    }
}
