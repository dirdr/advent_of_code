use std::{
    char,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Write},
};

use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self, input: &[String]) -> Answer {
        let map = Map::from_input(input);
        map.minimum_cost().0.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let map = Map::from_input(input);
        map.minimum_cost().1.len().into()
    }
}

#[allow(dead_code)]
#[derive(Clone, Default, PartialEq, Copy)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Start,
    End,
    Debug,
}

struct Map {
    map: Matrix<Tile>,
    starting_pos: Vec2<usize>,
}

#[derive(Eq, PartialEq, Clone)]
struct Node {
    cost: u64,
    pos: Vec2<usize>,
    dir: Cardinal,
    path: Vec<Vec2<usize>>,
}

impl Map {
    fn from_input(input: &[String]) -> Self {
        let map = Matrix::from_chars(input).map_to(Tile::from);
        let Some(starting_pos) = map.find(Tile::Start) else {
            panic!("Cannot find starting pos inside input");
        };
        Self { map, starting_pos }
    }

    fn minimum_cost(&self) -> (u64, HashSet<Vec2<usize>>) {
        let mut min_cost = u64::MAX;
        let mut heap = BinaryHeap::new();
        let mut all_paths = HashSet::<Vec2<usize>>::new();
        let start_node = Node {
            pos: self.starting_pos,
            dir: Cardinal::East,
            cost: 0,
            path: vec![self.starting_pos],
        };
        heap.push(start_node);

        let mut visited = HashMap::new();

        while let Some(Node {
            pos,
            dir,
            cost,
            path,
        }) = heap.pop()
        {
            if let Some(&prev) = visited.get(&(pos, dir)) {
                if cost > prev {
                    continue;
                }
            } else {
                visited.insert((pos, dir), cost);
            }

            if self.map[pos] == Tile::End {
                if min_cost == u64::MAX {
                    min_cost = cost;
                }
                all_paths.extend(path.iter());
            }

            let next_forward = dir.advance(pos.into());
            if let Some(&tile) = self.map.get(&next_forward) {
                let next_forward = next_forward.to_usize_unchecked();
                let mut next_path = path.clone();
                next_path.push(next_forward);
                if tile != Tile::Wall {
                    heap.push(Node {
                        pos: next_forward,
                        dir,
                        cost: cost + 1,
                        path: next_path,
                    })
                }
            }

            heap.push(Node {
                pos,
                dir: dir.turn_right(),
                cost: cost + 1000,
                path: path.clone(),
            });

            heap.push(Node {
                pos,
                dir: dir.turn_left(),
                cost: cost + 1000,
                path: path.clone(),
            });
        }
        (min_cost, all_paths)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Wall => f.write_char('#'),
            Tile::Start => f.write_char('S'),
            Tile::End => f.write_char('E'),
            Tile::Debug => f.write_char('O'),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.map).as_ref())
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day16;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_16_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(7036), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_16_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(45), answer);
    }
}
