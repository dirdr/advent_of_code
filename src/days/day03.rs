use std::collections::{HashMap, HashSet};

use crate::helper_lib::{answer::Answer, matrix::Matrix, solution::Solution, vec2::Vec2};

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut sum = 0;
        let parsed = parse(input);
        let grid = &parsed.grid;
        for y in 0..grid.rows {
            let mut number_buffer = String::new();
            let mut is_part_member = false;
            for x in 0..grid.cols {
                let pos = Vec2::new(x, y);
                let ch = grid[pos];
                let part = Part::from(ch);
                match part {
                    Part::Digit => {
                        number_buffer.push(ch);
                        if !parsed
                            .get_parts_positions_around(pos, &[Part::Gear, Part::Symbol])
                            .is_empty()
                        {
                            is_part_member = true;
                        }
                    }
                    _ => {
                        if !number_buffer.is_empty() {
                            let val = number_buffer.parse::<usize>().unwrap();
                            if is_part_member {
                                sum += val;
                            }
                            number_buffer.clear();
                            is_part_member = false;
                        }
                    }
                }
            }
            if !number_buffer.is_empty() && is_part_member {
                sum += number_buffer.parse::<usize>().unwrap();
            }
        }
        sum.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let grid = &parsed.grid;
        let mut map: HashMap<Vec2<usize>, Vec<usize>> = HashMap::new();
        for y in 0..grid.rows {
            let mut number_buffer = String::new();
            // use of an hashset to prevent same insert
            let mut gears_position: HashSet<Vec2<usize>> = HashSet::new();
            for x in 0..grid.cols {
                let pos = Vec2::new(x, y);
                let ch = grid[pos];
                let part = Part::from(ch);
                match part {
                    Part::Digit => {
                        number_buffer.push(ch);
                        gears_position.extend(
                            parsed
                                .get_parts_positions_around(pos, &[Part::Gear])
                                .iter()
                                .cloned(),
                        );
                    }
                    _ => {
                        if !number_buffer.is_empty() {
                            let parsed = number_buffer.parse::<usize>().unwrap();
                            for gp in gears_position.iter() {
                                map.entry(*gp)
                                    .and_modify(|e| e.push(parsed))
                                    .or_insert(vec![parsed]);
                            }
                            gears_position.clear();
                            number_buffer.clear();
                        }
                    }
                }
            }
            if !number_buffer.is_empty() {
                let parsed = number_buffer.parse::<usize>().unwrap();
                for gp in gears_position.iter() {
                    map.entry(*gp)
                        .and_modify(|e| e.push(parsed))
                        .or_insert(vec![parsed]);
                }
                gears_position.clear();
                number_buffer.clear();
            }
        }
        let sum = map
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .fold(0, |acc, (_, v)| acc + (v[0] * v[1]));
        sum.into()
    }
}

struct Parsed {
    grid: Matrix<char>,
}

fn parse(input: &[String]) -> Parsed {
    Parsed {
        grid: Matrix::from_chars(input),
    }
}

impl Parsed {
    fn get_parts_positions_around(&self, pos: Vec2<usize>, parts: &[Part]) -> Vec<Vec2<usize>> {
        let offsets: [Vec2<isize>; 8] = [
            Vec2::new(0, 1),
            Vec2::new(0, -1),
            Vec2::new(1, 0),
            Vec2::new(-1, 0),
            Vec2::new(-1, -1),
            Vec2::new(1, 1),
            Vec2::new(-1, 1),
            Vec2::new(1, -1),
        ];
        let mut positions = vec![];
        for &offset in &offsets {
            let new_pos = Vec2::<isize>::from(pos) + offset;
            let el = self.grid.get(new_pos);
            if let Some(&el) = el {
                for part in parts {
                    let cp = Part::from(el);
                    if cp == *part {
                        positions.push(Vec2::<usize>::try_from(new_pos).unwrap())
                    }
                }
            }
        }
        positions
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Part {
    Dot,
    Digit,
    Gear,
    Symbol,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            '.' => Part::Dot,
            '*' => Part::Gear,
            value if value.is_ascii_digit() => Part::Digit,
            _ => Part::Symbol,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day3;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_03_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day3.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4361), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_03_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day3.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(467835), answer);
    }
}
