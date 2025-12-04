use aoc_lib::{
    answer::Answer,
    directions::{Advance, Direction, ExtendedCardinal},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, input: &[String]) -> Answer {
        Shed::from_input(input).remove_accessible_rolls().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut shed = Shed::from_input(input);
        let mut total = 0;
        loop {
            let removed = shed.remove_accessible_rolls();
            if removed == 0 {
                break;
            }
            total += removed;
        }
        total.into()
    }
}

impl Shed {
    fn remove_accessible_rolls(&mut self) -> usize {
        let mut to_remove = vec![];

        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let pos = Vec2::new(x, y);
                if TileType::Paper == self.map[pos] {
                    let mut count = 0;
                    for dir in ExtendedCardinal::all_clockwise() {
                        let reached = Vec2::<isize>::from(pos);
                        let next = dir.advance(reached);
                        let Some(&tile) = self.map.get(&next) else {
                            continue;
                        };

                        if TileType::Paper == tile {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        to_remove.push(pos);
                    }
                }
            }
        }

        for &tr in &to_remove {
            self.map[tr] = TileType::Empty;
        }

        to_remove.len()
    }

    fn from_input(input: &[String]) -> Self {
        let map = Matrix::from_chars(input).map_to(TileType::from);
        Self { map }
    }
}

struct Shed {
    map: Matrix<TileType>,
}

#[derive(Default, Clone, Eq, PartialEq, Copy)]
enum TileType {
    Paper,

    #[default]
    Empty,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day4;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_04_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day4.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(13), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_04_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day4.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(43), answer);
    }
}
