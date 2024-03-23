use std::collections::HashMap;

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day14;

impl Solution for Day14 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut plateform = parse(input);
        plateform.tilt(Direction::North);
        plateform.score().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut plateform = parse(input);
        const ITERATIONS: usize = 1000000000;
        let mut seen = HashMap::new();
        for i in 0..ITERATIONS {
            if let Some(previous) = seen.get(&plateform) {
                if (ITERATIONS - i) % (i - previous) == 0 {
                    return plateform.score().into();
                }
            }
            seen.insert(plateform.clone(), i);
            plateform.tilt_cycle();
        }
        plateform.score().into()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Plateform {
    plateform: Matrix<char>,
}

impl Plateform {
    fn tilt_cycle(&mut self) {
        for direction in Direction::counter_clockwise_cycle() {
            self.tilt(direction);
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let offset = direction.to_offset();
        loop {
            let mut moved = false;
            for y in 0..self.plateform.rows {
                for x in 0..self.plateform.cols {
                    let pos = Vec2::new(x, y);
                    let current = self.plateform[pos];
                    if current != 'O' {
                        continue;
                    }
                    let new_position = Vec2::<isize>::from(pos) + offset;
                    let el_at_new_position = self.plateform.get(new_position);
                    if let Some(&np) = el_at_new_position {
                        if np != '.' {
                            continue;
                        }
                        self.plateform[Vec2::<usize>::try_from(new_position).unwrap()] = 'O';
                        self.plateform[pos] = '.';
                        moved = true;
                    }
                }
            }
            if !moved {
                break;
            }
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.plateform.rows {
            for x in 0..self.plateform.cols {
                let pos = Vec2::new(x, y);
                if self.plateform[pos] == 'O' {
                    score += self.plateform.rows - y;
                }
            }
        }
        score
    }
}

fn parse(input: &[String]) -> Plateform {
    Plateform {
        plateform: Matrix::from_chars(input),
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day14;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_14_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day14.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(136), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_14_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day14.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(64), answer);
    }
}
