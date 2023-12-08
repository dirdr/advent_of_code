use std::collections::{HashMap, HashSet};

use crate::helper_lib::{
    self, input,
    utils::{Matrix, Vec2},
};

#[derive(Clone)]
enum Part {
    Dot,
    Digit,
    Symbol,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            '.' => Part::Dot,
            value if value.is_digit(10) => Part::Digit,
            _ => Part::Symbol,
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let lines = input::read_file(&format!("{}day_3.txt", helper_lib::FILES_PREFIX))?;
    part_a(&lines)?;
    part_b(&lines)?;
    Ok(())
}

fn part_a(lines: &[String]) -> anyhow::Result<()> {
    let mut sum = 0;
    let mut matrix: Matrix<char> = Matrix::new(lines.len(), lines[0].len(), ' ');
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            *matrix.get_mut(row, col).unwrap() = ch;
        }
    }
    for row in 0..matrix.rows {
        let mut number_buffer = String::new();
        let mut is_part_member = false;
        for col in 0..matrix.cols {
            let ch = *matrix.get(row, col).unwrap();
            let part: Part = ch.into();
            match part {
                Part::Digit => {
                    number_buffer.push(ch);
                    if is_around_symbol(row, col, &matrix) {
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
    println!("{}", sum);
    Ok(())
}

fn is_around_symbol(row: usize, col: usize, matrix: &Matrix<char>) -> bool {
    let modifiers: [(i32, i32); 8] = [
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
        let new_row: i32 = row as i32 + row_mod;
        let new_col: i32 = col as i32 + col_mod;
        if new_row >= 0
            && new_row < matrix.rows as i32
            && new_col >= 0
            && new_col < matrix.cols as i32
        {
            let ch = *matrix.get(new_row as usize, new_col as usize).unwrap();
            let part: Part = ch.into();
            match part {
                Part::Symbol => return true,
                _ => (),
            }
        }
    }
    false
}

fn part_b(lines: &[String]) -> anyhow::Result<()> {
    Ok(())
}
