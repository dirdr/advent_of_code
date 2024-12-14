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
        map.seen().len().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let map = parse(input);
        map.seen().iter().filter(|p| map.is_loop(p)).count().into()
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
    fn seen(&self) -> HashSet<Vec2<usize>> {
        let mut pos = Vec2::<isize>::from(self.starting_pos);
        let mut dir = Cardinal::North;
        let mut visited: HashSet<Vec2<usize>> = HashSet::new();
        loop {
            visited.insert(Vec2::<usize>::try_from(&pos).unwrap());
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
        visited
    }

    fn is_loop(&self, obstacle: &Vec2<usize>) -> bool {
        let mut pos = Vec2::<isize>::from(self.starting_pos);
        let mut dir = Cardinal::North;
        let mut visited: HashSet<(Vec2<usize>, Cardinal)> = HashSet::new();
        loop {
            let Some(&ch) = self.grid.get(&pos) else {
                break;
            };
            if ch == '#' || pos == obstacle.into() {
                pos = dir.opposite().advance(pos);
                dir = dir.turn_right();
            }

            let current_state = (Vec2::<usize>::try_from(&pos).unwrap(), dir);

            if !visited.insert(current_state) {
                return true;
            }

            pos = dir.advance(pos);
        }
        false
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
