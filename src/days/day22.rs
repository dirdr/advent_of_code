use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::helper_lib::{answer::Answer, matrix::Matrix, solution::Solution, vec2::Vec2};

pub struct Day22;

impl Solution for Day22 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut playground = parse(input);
        // sort bricks by z value to start by the bottom
        playground
            .bricks
            .sort_unstable_by(|a, b| a.end.2.cmp(&b.end.2));
        let brick_at_levels = playground.release_bricks();
        let upper_to_base_bricks = get_upper_to_base_bricks(&playground.bricks, brick_at_levels);
        let critical = upper_to_base_bricks.values().filter(|b| b.len() == 1).fold(
            HashSet::new(),
            |mut acc: HashSet<Brick>, x| {
                acc.extend(x);
                acc
            },
        );
        (playground.bricks.len() - critical.len()).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut playground = parse(input);
        // sort bricks by z value to start by the bottom
        playground
            .bricks
            .sort_unstable_by(|a, b| a.end.2.cmp(&b.end.2));
        let brick_at_levels = playground.release_bricks();
        let upper_to_base_bricks = get_upper_to_base_bricks(&playground.bricks, brick_at_levels);

        let mut base_to_upper_bricks: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        for (brick, base) in upper_to_base_bricks.iter() {
            for base_brick in base.iter() {
                base_to_upper_bricks
                    .entry(*base_brick)
                    .or_default()
                    .insert(**brick);
            }
        }

        let mut total = 0;
        for brick in playground.bricks.iter() {
            if base_to_upper_bricks.get(brick).is_none() {
                continue;
            }
            let mut falling = HashSet::new();
            if let Some(upper) = base_to_upper_bricks.get(brick) {
                for upper_brick in upper {
                    if let Some(ub) = upper_to_base_bricks.get(upper_brick) {
                        if ub.len() == 1 {
                            falling.insert(*brick);
                        }
                    }
                }
            }
            let mut queue = VecDeque::from(falling.iter().cloned().collect_vec());
            falling.insert(*brick);
            while !queue.is_empty() {
                let upper = queue.pop_front().unwrap();
                if base_to_upper_bricks.get(&upper).is_none() {
                    continue;
                }
                let ups = base_to_upper_bricks
                    .get(&upper)
                    .unwrap()
                    .iter()
                    .filter(|v| !falling.contains(v))
                    .collect::<HashSet<_>>();
                for up in ups {
                    // if the brick is only supported by falling brick, this brick falls to
                    let base_bricks = upper_to_base_bricks.get(up).unwrap();
                    if base_bricks.iter().all(|support| falling.contains(support)) {
                        falling.insert(*up);
                        queue.push_back(*up);
                    }
                }
            }
            total += falling.len() - 1;
        }
        total.into()
    }
}

fn parse(input: &[String]) -> Playground {
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

        assert!(start[0] <= end[0]);
        assert!(start[1] <= end[1]);
        assert!(start[2] <= end[2]);

        bricks.push(Brick {
            start: (start[0], start[1], start[2]),
            end: (end[0], end[1], end[2]),
        });
    }
    let height_map = create_height_map(&bricks);
    Playground { bricks, height_map }
}

fn get_upper_to_base_bricks(
    bricks: &[Brick],
    brick_at_level: HashMap<usize, HashSet<Brick>>,
) -> HashMap<&Brick, HashSet<Brick>> {
    let mut out: HashMap<&Brick, HashSet<Brick>> = HashMap::new();
    for brick in bricks {
        out.insert(brick, HashSet::new());
        if let Some(others) = brick_at_level.get(&(brick.start.2 - 1)) {
            // for bricks that are in the z level below us
            for other in others {
                if brick.intersect_with(other) {
                    out.entry(brick).or_default().insert(*other);
                }
            }
        }
    }
    out
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

struct Playground {
    bricks: Vec<Brick>,
    height_map: HeightMap,
}

impl Playground {
    fn release_bricks(&mut self) -> HashMap<usize, HashSet<Brick>> {
        let mut out: HashMap<usize, HashSet<Brick>> = HashMap::new();
        for brick in self.bricks.iter_mut() {
            let area = brick.get_area();
            let below_z = self.height_map.get_heighest_z(&area);
            // set the brick on top of the max height for area of new brick
            brick.set_new_z(below_z + 1);
            self.height_map.set_max_height(&area, brick.end.2);
            out.entry(brick.end.2).or_default().insert(*brick);
        }
        out
    }
}

#[derive(Debug)]
struct Area {
    tl: Vec2<usize>,
    br: Vec2<usize>,
}

impl HeightMap {
    fn get_heighest_z(&self, area: &Area) -> usize {
        let mut max = 0;
        for x in area.tl.x..(area.br.x + 1) {
            for y in area.tl.y..(area.br.y + 1) {
                max = std::cmp::max(max, self.map[Vec2::new(x, y)]);
            }
        }
        max
    }

    fn set_max_height(&mut self, area: &Area, z: usize) {
        for x in area.tl.x..(area.br.x + 1) {
            for y in area.tl.y..(area.br.y + 1) {
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
    fn get_area(&self) -> Area {
        Area {
            tl: Vec2::new(self.start.0, self.start.1),
            br: Vec2::new(self.end.0, self.end.1),
        }
    }

    fn intersect_with(&self, other: &Brick) -> bool {
        if self.end.0 < other.start.0 || self.start.0 > other.end.0 {
            return false;
        }
        if self.end.1 < other.start.1 || self.start.1 > other.end.1 {
            return false;
        }
        true
    }

    fn set_new_z(&mut self, new_bottom: usize) {
        let delta = self.start.2 - new_bottom;
        self.start.2 -= delta;
        self.end.2 -= delta;
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
        assert_eq!(<i32 as Into<Answer>>::into(7), answer);
    }
}
