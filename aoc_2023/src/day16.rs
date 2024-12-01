use std::collections::HashSet;

use aoc_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        grid.simulate(Vec2::new(-1, 0), Direction::East).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        let mut max = 0;
        for y in 0..grid.grid.rows as isize {
            max = max.max(grid.simulate(Vec2::new(-1, y), Direction::East));
            max = max.max(grid.simulate(Vec2::new(grid.grid.cols as isize, y), Direction::West));
        }
        for x in 0..grid.grid.cols as isize {
            max = max.max(grid.simulate(Vec2::new(x, -1), Direction::South));
            max = max.max(grid.simulate(Vec2::new(x, grid.grid.rows as isize), Direction::North));
        }
        max.into()
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
    fn simulate(&self, initial_pos: Vec2<isize>, initial_direction: Direction) -> usize {
        let mut energized: HashSet<Vec2<isize>> = HashSet::new();
        let mut rays = vec![Ray {
            pos: initial_pos,
            direction: initial_direction,
        }];
        while !rays.is_empty() {
            let mut next_rays = vec![];
            for ray in rays.iter_mut() {
                let pos = ray.pos + ray.direction.to_offset();
                if !self.grid.contains(pos) {
                    continue;
                }
                let tile = self.grid[Vec2::<usize>::try_from(pos).unwrap()];
                let not_seen = energized.insert(pos);

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
                    Tile::Horizontal if not_seen => {
                        next_rays.push(Ray {
                            direction: Direction::East,
                            pos,
                        });
                        next_rays.push(Ray {
                            direction: Direction::West,
                            pos,
                        });
                    }
                    Tile::Vertical if not_seen => {
                        next_rays.push(Ray {
                            direction: Direction::North,
                            pos,
                        });
                        next_rays.push(Ray {
                            direction: Direction::South,
                            pos,
                        });
                    }
                    _ => (),
                };
            }
            std::mem::swap(&mut rays, &mut next_rays);
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
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day16;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            crate::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(46), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            crate::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(51), answer);
    }
}
