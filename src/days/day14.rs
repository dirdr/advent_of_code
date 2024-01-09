use std::collections::HashMap;

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution,
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
        let iterations = 1000000000;
        let mut seen = HashMap::new();
        for i in 0..iterations {
            if let Some(previous) = seen.get(&plateform) {
                if (iterations - i) % (i - previous) == 0 {
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
                    let pos = (y, x);
                    let current = self.plateform[pos];
                    if current != 'O' {
                        continue;
                    }
                    let new_position = (y as isize + offset.1, x as isize + offset.0);
                    let el_at_new_position = self.plateform.get(new_position.1, new_position.0);
                    if let Some(&np) = el_at_new_position {
                        if np != '.' {
                            continue;
                        }
                        let new_position = (new_position.0 as usize, new_position.1 as usize);
                        self.plateform[new_position] = 'O';
                        self.plateform[(y, x)] = '.';
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
                if self.plateform[(y, x)] == 'O' {
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
    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_14_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day14.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(136), answer);
    }

    #[test]
    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_14_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day14.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(64), answer);
    }
}
