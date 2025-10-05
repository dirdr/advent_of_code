use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self, input: &[String]) -> Answer {
        let racetrack = RaceTrack::from_input(input);
        racetrack.solve(100, 2).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let racetrack = RaceTrack::from_input(input);
        racetrack.solve(100, 20).into()
    }
}

impl RaceTrack {
    fn solve(&self, threshold: usize, cheat_range: usize) -> usize {
        let path = self.path();
        let mut cheats = HashSet::new();

        let mut path_indices = HashMap::new();
        for (i, &pos) in path.iter().enumerate() {
            path_indices.insert(pos, i);
        }

        for (i, &cell) in path.iter().enumerate() {
            for dx in -(cheat_range as isize)..=(cheat_range as isize) {
                for dy in -(cheat_range as isize)..=(cheat_range as isize) {
                    let manhattan = dx.abs() + dy.abs();

                    if manhattan < 0 || manhattan > cheat_range as isize {
                        continue;
                    }

                    let new_x = cell.x as isize + dx;
                    let new_y = cell.y as isize + dy;

                    if new_x < 0 || new_y < 0 {
                        continue;
                    }

                    let other = Vec2::<usize> {
                        x: new_x as usize,
                        y: new_y as usize,
                    };

                    if let Some(&other_cell_index) = path_indices.get(&other) {
                        if other_cell_index <= i {
                            continue;
                        }

                        let normal_path_distance = other_cell_index - i;
                        let manhattan_distance = manhattan as usize;
                        let gained = normal_path_distance - manhattan_distance;

                        if gained >= threshold {
                            cheats.insert(vec![cell, other]);
                        }
                    }
                }
            }
        }
        cheats.len()
    }

    fn path(&self) -> Vec<Vec2<usize>> {
        let mut pq = BinaryHeap::new();
        pq.push(State {
            cost: 0,
            pos: self.starting_pos,
        });
        let mut visited = HashSet::new();
        let mut parent: HashMap<Vec2<usize>, Vec2<usize>> = HashMap::new();

        while let Some(State { cost, pos }) = pq.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if pos == self.ending_pos {
                let mut path = vec![pos];
                let mut current = pos;
                while let Some(&p) = parent.get(&current) {
                    path.push(p);
                    current = p;
                }
                path.reverse();
                return path;
            }

            for dir in Cardinal::all_clockwise() {
                let next = dir.advance(pos.into());
                let Some(&front) = self.map.get(&next) else {
                    continue;
                };
                if front == Tile::Wall {
                    continue;
                }
                let next = Vec2::<usize>::try_from(next).unwrap();
                if visited.contains(&next) {
                    continue;
                }
                parent.insert(next, pos);
                pq.push(State {
                    cost: cost + 1,
                    pos: next,
                });
            }
        }
        unreachable!()
    }

    fn from_input(input: &[String]) -> Self {
        let map = Matrix::from_chars(input).map_to(Tile::from);
        let Some(starting_pos) = map.find(Tile::Start) else {
            panic!("Cannot find starting pos inside input");
        };
        let Some(ending_pos) = map.find(Tile::End) else {
            panic!("Cannot find ending pos inside input");
        };
        Self {
            map,
            starting_pos,
            ending_pos,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Vec2<usize>,
}

struct RaceTrack {
    map: Matrix<Tile>,
    starting_pos: Vec2<usize>,
    ending_pos: Vec2<usize>,
}

#[derive(Clone, Default, PartialEq, Copy)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day20::RaceTrack;

    use aoc_lib::{answer::Answer, input};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_20_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let racetrack = RaceTrack::from_input(&input);
        let saved = racetrack.solve(20, 2).into();
        assert_eq!(<i32 as Into<Answer>>::into(5), saved);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_20_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let racetrack = RaceTrack::from_input(&input);
        let saved = racetrack.solve(74, 20).into();
        assert_eq!(<i32 as Into<Answer>>::into(7), saved);
    }
}
