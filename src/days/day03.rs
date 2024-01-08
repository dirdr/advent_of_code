use std::collections::{HashMap, HashSet};

use crate::helper_lib::{answer::Answer, matrix::Matrix, solution::Solution};

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, lines: &[String]) -> Answer {
        let mut sum = 0;
        let mut matrix: Matrix<char> = Matrix::new(lines.len(), lines[0].len(), ' ');
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                matrix[(x, y)] = ch;
            }
        }
        for y in 0..matrix.rows {
            let mut number_buffer = String::new();
            let mut is_part_member = false;
            for x in 0..matrix.cols {
                let ch = matrix[(x, y)];
                let part = Part::from(ch);
                match part {
                    Part::Digit => {
                        number_buffer.push(ch);
                        if is_around_symbol(y, x, &matrix) {
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

    fn part_b(&self, lines: &[String]) -> Answer {
        let mut matrix: Matrix<char> = Matrix::new(lines.len(), lines[0].len(), ' ');
        let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                matrix[(x, y)] = ch;
            }
        }
        for y in 0..matrix.rows {
            let mut number_buffer = String::new();
            // use of an hashset to prevent multiple, same gear insert
            let mut gears_position: HashSet<(usize, usize)> = HashSet::new();
            for x in 0..matrix.cols {
                let ch = matrix[(x, y)];
                let part = Part::from(ch);
                match part {
                    Part::Digit => {
                        number_buffer.push(ch);
                        let digit_gears_position = nearby_gears_position(y, x, &matrix);
                        for pos in digit_gears_position {
                            gears_position.insert(pos);
                        }
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

#[derive(Clone)]
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

fn is_around_symbol(row: usize, col: usize, matrix: &Matrix<char>) -> bool {
    let modifiers: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    for &(row_mod, col_mod) in &modifiers {
        let new_row = row as isize + row_mod;
        let new_col = col as isize + col_mod;
        let el = matrix.get(new_row, new_col);
        if let Some(&el) = el {
            match Part::from(el) {
                Part::Symbol | Part::Gear => return true,
                _ => (),
            }
        }
    }
    false
}

fn nearby_gears_position(row: usize, col: usize, matrix: &Matrix<char>) -> Vec<(usize, usize)> {
    let mut answer = vec![];
    let modifiers: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    for &(row_mod, col_mod) in &modifiers {
        let new_row = row as isize + row_mod;
        let new_col = col as isize + col_mod;
        let ch = matrix.get(new_row, new_col);
        if let Some(&ch) = ch {
            let part = Part::from(ch);
            if let Part::Gear = part {
                answer.push((new_row as usize, new_col as usize));
            }
        }
    }
    answer
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day3;

    #[test]
    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_03_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day3.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4361), answer);
    }

    #[test]
    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_03_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day3.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(467835), answer);
    }
}
