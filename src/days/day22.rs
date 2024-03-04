use std::collections::HashSet;

use crate::helper_lib::{answer::Answer, matrix::Matrix, solution::Solution, vec2::Vec2};

pub struct Day22;

impl Solution for Day22 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut bricks = parse(input);
        // sort bricks by z value to start by the bottom
        bricks.sort_unstable_by(|a, b| a.end.2.cmp(&b.end.2));
        let mut height_map = create_height_map(&bricks);
        release_bricks(&mut bricks, &mut height_map);
        let base_bricks = get_base_bricks(&bricks);
        println!("{:?}", base_bricks);
        (bricks.len() - base_bricks.len()).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

fn parse(input: &[String]) -> Vec<Brick> {
    let mut bricks = vec![];
    for line in input {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let end = end
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        bricks.push(Brick {
            start: (start[0], start[1], start[2]),
            end: (end[0], end[1], end[2]),
        });
    }
    bricks
}

fn release_bricks(bricks: &mut Vec<Brick>, height_map: &mut HeightMap) {
    println!("{:?}", bricks);
    for brick in bricks.iter_mut() {
        let brick_area = brick.get_area_coords();
        let z = height_map.get_heighest_z_in_area(brick_area.0, brick_area.1);
        brick.start.2 = z + 1;
        height_map.update_max_height_in_area(brick_area.0, brick_area.1, brick.end.2);
    }
}

fn get_base_bricks(bricks: &Vec<Brick>) -> HashSet<Brick> {
    println!("{:?}", bricks);
    let mut set = HashSet::new();
    for brick in bricks.iter() {
        for other in bricks.iter() {
            //println!("self z : {}, other z: {}", brick.start.2, other.end.2);
            if brick != other && brick.start.2 >= other.end.2 && brick.intersect_with(other) {
                set.insert(*other);
            }
        }
    }
    set
}

fn create_height_map(bricks: &Vec<Brick>) -> HeightMap {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for brick in bricks {
        max_x = std::cmp::max(max_x, brick.end.0);
        max_y = std::cmp::max(max_y, brick.end.1);
        min_x = std::cmp::min(min_x, brick.start.0);
        min_y = std::cmp::min(min_y, brick.start.1);
    }
    HeightMap {
        map: Matrix::new(max_y - min_y + 1, max_x - min_x + 1, 0usize),
    }
}

struct HeightMap {
    map: Matrix<usize>,
}

impl HeightMap {
    fn get_heighest_z_in_area(&self, tl: Vec2<usize>, br: Vec2<usize>) -> usize {
        let mut max = 0;
        for x in tl.x..br.x {
            for y in tl.y..br.y {
                max = std::cmp::max(max, self.map[Vec2::new(x, y)]);
            }
        }
        max
    }

    fn update_max_height_in_area(&mut self, tl: Vec2<usize>, br: Vec2<usize>, z: usize) {
        for x in tl.x..br.x {
            for y in tl.y..br.y {
                self.map[Vec2::new(x, y)] = z;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn get_area_coords(&self) -> (Vec2<usize>, Vec2<usize>) {
        (
            Vec2::new(self.start.0, self.start.1),
            Vec2::new(self.end.0, self.end.1),
        )
    }

    fn intersect_with(&self, other: &Brick) -> bool {
        self.start.0 < other.end.0
            && self.end.0 > other.start.0
            && self.start.1 > other.end.1
            && self.end.1 < other.start.1
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day22;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_22_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day22.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(5), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_22_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day22.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(16733044), answer);
    }
}
