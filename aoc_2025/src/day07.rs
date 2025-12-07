use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use aoc_lib::{
    answer::Answer,
    directions::{self, Advance},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day7;

impl Solution for Day7 {
    fn part_a(&self, input: &[String]) -> Answer {
        TachyonManifold::from_input(input).part_a().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        TachyonManifold::from_input(input).part_b().into()
    }
}

struct TachyonManifold {
    diagram: Matrix<Tile>,
}

#[derive(Copy, Clone, Default, PartialEq)]
enum Tile {
    Start,
    Splitter,
    Beam,

    #[default]
    Empty,
}

impl TachyonManifold {
    fn part_a(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start_pos = self.find_starting_pos();

        queue.push_back(start_pos);
        visited.insert(start_pos);

        let mut count = 0;

        while let Some(pos) = queue.pop_front() {
            let tile = self.diagram[pos];

            if tile == Tile::Splitter {
                count += 1;

                if pos.x > 0 {
                    let left = Vec2::new(pos.x - 1, pos.y);
                    if visited.insert(left) {
                        queue.push_back(left);
                    }
                }

                let right = Vec2::new(pos.x + 1, pos.y);
                if right.x < self.diagram.cols && visited.insert(right) {
                    queue.push_back(right);
                }
            } else {
                if tile == Tile::Empty {
                    self.diagram[pos] = Tile::Beam;
                }

                let next_pos = directions::Cardinal::South.advance(Vec2::<isize>::from(pos));
                if !self.diagram.contains(&next_pos) {
                    continue;
                }
                let next_pos_usize = Vec2::<usize>::try_from(next_pos).unwrap();
                if visited.insert(next_pos_usize) {
                    queue.push_back(next_pos_usize);
                }
            }
        }

        count
    }

    fn part_b(&self) -> usize {
        fn dfs(
            diagram: &Matrix<Tile>,
            memo: &mut HashMap<Vec2<usize>, usize>,
            pos: Vec2<usize>,
        ) -> usize {
            if let Some(&cached) = memo.get(&pos) {
                return cached;
            }

            let tile = diagram[pos];
            let result = if tile == Tile::Splitter {
                let mut total = 0;

                if pos.x > 0 {
                    total += dfs(diagram, memo, Vec2::new(pos.x - 1, pos.y));
                }

                if pos.x + 1 < diagram.cols {
                    total += dfs(diagram, memo, Vec2::new(pos.x + 1, pos.y));
                }

                total
            } else if pos.y + 1 < diagram.rows {
                dfs(diagram, memo, Vec2::new(pos.x, pos.y + 1))
            } else {
                1
            };

            memo.insert(pos, result);
            result
        }

        dfs(&self.diagram, &mut HashMap::new(), self.find_starting_pos())
    }

    fn find_starting_pos(&self) -> Vec2<usize> {
        for y in 0..self.diagram.rows {
            for x in 0..self.diagram.cols {
                let pos = Vec2::new(x, y);
                if self.diagram[pos] == Tile::Start {
                    return pos;
                }
            }
        }
        unreachable!()
    }

    fn from_input(input: &[String]) -> Self {
        Self {
            diagram: Matrix::from_chars(input).map_to(Tile::from),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            '^' => Tile::Splitter,
            '.' => Tile::Empty,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => f.write_str("S"),
            Tile::Splitter => f.write_str("^"),
            Tile::Beam => f.write_str("|"),
            Tile::Empty => f.write_str("."),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day7;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_07_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day7.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(21), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_07_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day7.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(40), answer);
    }
}
