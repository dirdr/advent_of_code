use std::collections::HashSet;

use aoc_lib::{
    answer::Answer,
    directions::{Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        grid.simulate(Vec2::new(-1, 0), Cardinal::East).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        let mut max = 0;
        for y in 0..grid.grid.rows as isize {
            max = max.max(grid.simulate(Vec2::new(-1, y), Cardinal::East));
            max = max.max(grid.simulate(Vec2::new(grid.grid.cols as isize, y), Cardinal::West));
        }
        for x in 0..grid.grid.cols as isize {
            max = max.max(grid.simulate(Vec2::new(x, -1), Cardinal::South));
            max = max.max(grid.simulate(Vec2::new(x, grid.grid.rows as isize), Cardinal::North));
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
    fn simulate(&self, initial_pos: Vec2<isize>, initial_direction: Cardinal) -> usize {
        let mut energized: HashSet<Vec2<isize>> = HashSet::new();
        let mut rays = vec![Ray {
            pos: initial_pos,
            direction: initial_direction,
        }];
        while !rays.is_empty() {
            let mut next_rays = vec![];
            for ray in rays.iter_mut() {
                let pos = ray.pos + ray.direction.to_offset();
                if !self.grid.contains(&pos) {
                    continue;
                }
                let tile = self.grid[Vec2::<usize>::try_from(&pos).unwrap()];
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
                            Cardinal::North => Cardinal::East,
                            Cardinal::East => Cardinal::North,
                            Cardinal::South => Cardinal::West,
                            Cardinal::West => Cardinal::South,
                        };
                        next_rays.push(ray.clone());
                    }
                    Tile::LeftReflector => {
                        ray.direction = match ray.direction {
                            Cardinal::North => Cardinal::West,
                            Cardinal::East => Cardinal::South,
                            Cardinal::South => Cardinal::East,
                            Cardinal::West => Cardinal::North,
                        };
                        next_rays.push(ray.clone());
                    }
                    Tile::Horizontal if not_seen => {
                        next_rays.push(Ray {
                            direction: Cardinal::East,
                            pos,
                        });
                        next_rays.push(Ray {
                            direction: Cardinal::West,
                            pos,
                        });
                    }
                    Tile::Vertical if not_seen => {
                        next_rays.push(Ray {
                            direction: Cardinal::North,
                            pos,
                        });
                        next_rays.push(Ray {
                            direction: Cardinal::South,
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
    direction: Cardinal,
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
    fn matching_direction(&self, direction: &Cardinal) -> bool {
        matches!(
            (self, direction),
            (Tile::Horizontal, Cardinal::East | Cardinal::West)
                | (Tile::Vertical, Cardinal::North | Cardinal::South)
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
        let input =
            input::read_file(&format!("{}day_16_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(46), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_16_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(51), answer);
    }
}
