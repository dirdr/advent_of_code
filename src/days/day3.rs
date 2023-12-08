use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use itertools::Itertools;

use crate::helper_lib::{
    self, input,
    utils::{Matrix, Vec2},
};

const SYMBOLS: [char; 5] = ['+', '-', '&', '@', '#'];

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
                        println!("{val}");
                        if is_part_member {
                            sum += val;
                        }
                    }
                    number_buffer.clear();
                    is_part_member = false;
                }
            }
        }
    }
    println!("{}", sum);
    Ok(())
}

fn is_around_symbol(row: usize, col: usize, matrix: &Matrix<char>) -> bool {
    let modifiers: [(i32, i32); 6] = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (1, 1)];
    for &(row_mod, col_mod) in &modifiers {
        let new_row = isize::try_from(row)
            .ok()
            .and_then(|r| r.checked_add(row_mod as isize))
            .and_then(|r| usize::try_from(r).ok());
        let new_col = isize::try_from(col)
            .ok()
            .and_then(|c| c.checked_add(col_mod as isize))
            .and_then(|c| usize::try_from(c).ok());
        if let (Some(nr), Some(nc)) = (new_row, new_col) {
            if nr < matrix.rows && nc < matrix.cols {
                let ch = *matrix.get(row, col).unwrap();
                let part: Part = ch.into();
                match part {
                    Part::Symbol => return true,
                    _ => (),
                }
            }
        }
    }
    false
}

fn part_b(lines: &[String]) -> anyhow::Result<()> {
    Ok(())
}
