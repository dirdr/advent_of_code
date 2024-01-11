use std::collections::HashSet;

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        let mut rays = vec![
            Ray {
                direction: Direction::East,
                pos: Vec2::new(-1, 0)
            };
            1
        ];
        grid.simulate(&mut rays).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

struct Grid {
    matrix: Matrix<char>,
}

impl Grid {
    fn simulate(&self, rays: &mut Vec<Ray>) -> usize {
        let mut energized: HashSet<Vec2<isize>> = HashSet::new();
        while !rays.is_empty() {
            let mut to_remove = vec![];
            let mut to_add = vec![];
            for (i, ray) in rays.iter_mut().enumerate() {
                let pos = Vec2::new(ray.pos.x, ray.pos.y);
                if let Some(&p) = self.matrix.get(pos.y, pos.x) {
                    energized.insert(pos.clone());
                    match p {
                        '.' => {}
                        '/' => {}
                        '\\' => {}
                        '-' => {}
                        '|' => {}
                        _ => unreachable!(),
                    };
                } else {
                    to_remove.push(i);
                }
            }
            for ta in to_add {
                rays.push(ta);
            }
            for tr in to_remove {
                rays.remove(tr);
            }
        }
        energized.len()
    }
}

fn parse(input: &[String]) -> Grid {
    Grid {
        matrix: Matrix::from_chars(input),
    }
}

#[derive(Clone, Debug)]
struct Ray {
    direction: Direction,
    pos: Vec2<isize>,
}

impl Ray {
    fn new(direction: Direction, pos: Vec2<isize>) -> Self {
        Self { direction, pos }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day16;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(46), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(145), answer);
    }
}
