use std::fmt::{Display, Write};

use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};
use itertools::Itertools;

pub struct Day15;

impl Solution for Day15 {
    fn part_a(&self, input: &[String]) -> Answer {
        Problem::from_input(input).simulate().solve().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut problem = Problem::from_input(input);
        if let MapType::Simple(ref mut simple) = problem.map {
            problem.map = MapType::Expanded(simple.expand());
        }
        problem.simulate().solve().into()
    }
}

#[derive(PartialEq, Eq)]
enum MapType {
    Simple(Simple),
    Expanded(Expanded),
}

struct Problem {
    map: MapType,
    input_sequence: Vec<Cardinal>,
}

#[derive(PartialEq, Eq)]
struct Simple {
    map: Matrix<Tile>,
}

#[derive(PartialEq, Eq)]
struct Expanded {
    map: Matrix<Tile>,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Robot,
    Box,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Problem {
    fn from_input(input: &[String]) -> Self {
        let (map, input) = input.split(|l| l.is_empty()).collect_tuple().unwrap();
        let map = Matrix::from_chars(map).map_to(Tile::from);
        let input_sequence = input
            .join("")
            .chars()
            .map(get_cardinal_from_char)
            .collect::<Vec<Cardinal>>();
        Self {
            map: MapType::Simple(Simple { map }),
            input_sequence,
        }
    }

    fn simulate(mut self) -> Self {
        match self.map {
            MapType::Simple(ref mut simple) => {
                simple.simulate(&self.input_sequence);
            }
            MapType::Expanded(ref mut expanded) => {
                expanded.simulate(&self.input_sequence);
            }
        };
        self
    }

    fn solve(&self) -> u64 {
        let coordinates = match self.map {
            MapType::Simple(ref simple) => simple.get_gps_coordinates(),
            MapType::Expanded(ref expanded) => expanded.get_gps_coordinates(),
        };
        coordinates.iter().sum::<u64>()
    }
}

impl Simple {
    fn simulate(&mut self, input_sequence: &[Cardinal]) {
        let mut pos = self.map.find(Tile::Robot).unwrap();
        'outer: for input in input_sequence {
            let next = input.advance(pos.into());
            let Some(&front) = self.map.get(&next) else {
                continue;
            };
            if front == Tile::Wall {
                continue;
            }
            if front == Tile::Box {
                let mut peek = next;
                loop {
                    peek = input.advance(peek);
                    let Some(&tile) = self.map.get(&peek) else {
                        continue 'outer;
                    };
                    if tile == Tile::Wall {
                        continue 'outer;
                    }
                    let peek = Vec2::<usize>::try_from(peek).unwrap();
                    if tile == Tile::Empty {
                        self.map[peek] = Tile::Box;
                        break;
                    }
                }
            }
            let next = Vec2::<usize>::try_from(next).unwrap();
            self.map[pos] = Tile::Empty;
            self.map[next] = Tile::Robot;
            pos = next;
        }
    }

    fn get_gps_coordinates(&self) -> Vec<u64> {
        let mut coordinates = vec![];
        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let pos = Vec2::new(x, y);
                if self.map[pos] == Tile::Box {
                    coordinates.push((100 * y + x) as u64);
                }
            }
        }
        coordinates
    }

    fn expand(&self) -> Expanded {
        let mut map = Matrix::new(self.map.rows, self.map.cols * 2, Tile::Empty);
        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let old_pos = Vec2::new(x, y);
                let new_pos = Vec2::new(x * 2, y);
                match self.map[old_pos] {
                    Tile::Robot => {
                        map[new_pos] = Tile::Robot;
                    }
                    Tile::Box => {
                        map[new_pos] = Tile::BoxLeft;
                        map[new_pos + Vec2::new(1, 0)] = Tile::BoxRight;
                    }
                    Tile::Wall => {
                        map[new_pos] = Tile::Wall;
                        map[new_pos + Vec2::new(1, 0)] = Tile::Wall;
                    }
                    Tile::Empty => (),
                    _ => unreachable!(),
                }
            }
        }
        Expanded { map }
    }
}

impl Expanded {
    fn simulate(&mut self, instructions: &[Cardinal]) {
        let mut pos = self.map.find(Tile::Robot).unwrap();
        for instruction in instructions {
            let next = instruction
                .advance(Vec2::<isize>::from(pos))
                .to_usize_unchecked();
            if self.can_push(next, *instruction) {
                self.map[pos] = Tile::Empty;
                self.push(next, *instruction);
                self.map[next] = Tile::Robot;
                pos = next;
            }
        }
    }

    fn can_push(&self, pos: Vec2<usize>, dir: Cardinal) -> bool {
        let value = self.map[pos];

        match value {
            Tile::Empty => return true,
            Tile::Wall => return false,
            Tile::BoxLeft | Tile::BoxRight => {}
            Tile::Robot | Tile::Box => unreachable!(),
        }

        let other_box = match value {
            Tile::BoxLeft => pos + Vec2::new(1, 0),
            Tile::BoxRight => pos - Vec2::new(1, 0),
            _ => unreachable!(),
        };

        let new_a = dir.advance(pos.into());
        let new_b = dir.advance(other_box.into());

        if !(self.map.contains(&new_a) && self.map.contains(&new_b)) {
            return false;
        }

        let new_a = new_a.to_usize_unchecked();
        let new_b = new_b.to_usize_unchecked();

        let immediate = self.map[new_a] == Tile::Empty && self.map[new_b] == Tile::Empty;
        let futur = (new_a == other_box || self.can_push(new_a, dir))
            && (new_b == pos || self.can_push(new_b, dir));

        immediate || futur
    }

    fn push(&mut self, pos: Vec2<usize>, dir: Cardinal) -> bool {
        if !self.can_push(pos, dir) {
            return false;
        }
        let value = self.map[pos];
        if value == Tile::Empty {
            return true;
        }

        let other_box = match value {
            Tile::BoxLeft => pos + Vec2::new(1, 0),
            Tile::BoxRight => pos - Vec2::new(1, 0),
            _ => unreachable!(),
        };

        let other_value = self.map[other_box];

        let new_a = dir.advance(pos.into());
        let new_b = dir.advance(other_box.into());

        if !(self.map.contains(&new_a) && self.map.contains(&new_b)) {
            return false;
        }

        let new_a = new_a.to_usize_unchecked();
        let new_b = new_b.to_usize_unchecked();

        let immediate = self.map[new_a] == Tile::Empty && self.map[new_b] == Tile::Empty;
        let futur_a = new_a == other_box || self.push(new_a, dir);
        let futur_b = new_b == pos || self.push(new_b, dir);

        if !immediate && (!futur_a || !futur_b) {
            return false;
        }

        self.map[new_a] = value;
        self.map[pos] = Tile::Empty;
        self.map[new_b] = other_value;
        if other_box != new_a {
            self.map[other_box] = Tile::Empty;
        }
        true
    }

    fn get_gps_coordinates(&self) -> Vec<u64> {
        let mut coordinates = vec![];
        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let pos = Vec2::new(x, y);
                if self.map[pos] == Tile::Box || self.map[pos] == Tile::BoxLeft {
                    coordinates.push((100 * y + x) as u64);
                }
            }
        }
        coordinates
    }
}

fn get_cardinal_from_char(ch: char) -> Cardinal {
    match ch {
        '^' => Cardinal::North,
        '>' => Cardinal::East,
        'v' => Cardinal::South,
        '<' => Cardinal::West,
        _ => unreachable!(),
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '@' => Tile::Robot,
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('.'),
            Self::Wall => f.write_char('#'),
            Self::Robot => f.write_char('@'),
            Self::Box => f.write_char('O'),
            Self::BoxLeft => f.write_char('['),
            Self::BoxRight => f.write_char(']'),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day15;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_15_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day15.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(10092), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_15_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day15.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(9021), answer);
    }
}
