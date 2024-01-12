use std::{collections::HashSet, default};

use rayon::vec;

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
    grid: Matrix<Tile>,
}

fn parse(input: &[String]) -> Grid {
    Grid {
        grid: Matrix::from_chars(input).map_to(Tile::from),
    }
}

impl Grid {
    fn simulate(&self, rays: &mut Vec<Ray>) -> usize {
        let mut energized: HashSet<Vec2<isize>> = HashSet::new();
        let mut lc = 0;
        while !rays.is_empty() {
            println!("{:?}", rays);
            lc += 1;
            if lc == 100 {
                return 0;
            }
            let mut next_rays = vec![];
            for ray in rays.iter_mut() {
                let pos = ray.pos + ray.direction.to_offset();

                if !self.grid.contains(pos) {
                    continue;
                }

                let tile = self.grid[Vec2::<usize>::try_from(pos).unwrap()];

                if tile == Tile::Empty || tile.matching_direction(&ray.direction) {
                    ray.pos = pos;
                    next_rays.push(ray.clone());
                    continue;
                }

                ray.pos = pos;

                match tile {
                    Tile::RightReflector => {
                        ray.direction = match ray.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::North,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                        };
                        next_rays.push(ray.clone());
                    }
                    Tile::LeftReflector => {
                        ray.direction = match ray.direction {
                            Direction::North => Direction::West,
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                        };
                        next_rays.push(ray.clone());
                    }
                    Tile::Horizontal => {
                        next_rays.push(Ray {
                            direction: Direction::East,
                            pos: pos,
                        });
                        next_rays.push(Ray {
                            direction: Direction::West,
                            pos: pos,
                        });
                    }
                    Tile::Vertical => {
                        next_rays.push(Ray {
                            direction: Direction::North,
                            pos: pos,
                        });
                        next_rays.push(Ray {
                            direction: Direction::South,
                            pos: pos,
                        });
                    }
                    _ => (),
                };
            }
            std::mem::swap(rays, &mut next_rays);
        }
        energized.len()
    }
}

#[derive(Clone, Debug)]
struct Ray {
    direction: Direction,
    pos: Vec2<isize>,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Tile {
    RightReflector,
    LeftReflector,
    Horizontal,
    Vertical,
    #[default]
    Empty,
}

impl Tile {
    fn matching_direction(&self, direction: &Direction) -> bool {
        matches!(
            (self, direction),
            (Tile::Horizontal, Direction::East | Direction::West)
                | (Tile::Vertical, Direction::North | Direction::South)
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::RightReflector,
            '\\' => Tile::LeftReflector,
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            _ => unreachable!(),
        }
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
