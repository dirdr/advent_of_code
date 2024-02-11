use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use super::vec2::Vec2;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Index<Vec2<usize>> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        &self.data[index.y * self.cols + index.x]
    }
}

impl<T> IndexMut<Vec2<usize>> for Matrix<T> {
    fn index_mut(&mut self, index: Vec2<usize>) -> &mut T {
        &mut self.data[index.y * self.cols + index.x]
    }
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(rows: usize, cols: usize, inital_value: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![inital_value; rows * cols],
        }
    }

    pub fn get(&self, pos: Vec2<isize>) -> Option<&T> {
        Vec2::<usize>::try_from(pos)
            .ok()
            .and_then(|p| self.in_range(p).then(|| &self[p]))
    }

    pub fn get_mut(&mut self, pos: Vec2<isize>) -> Option<&mut T> {
        Vec2::<usize>::try_from(pos)
            .ok()
            .and_then(|p| self.in_range(p).then(|| &mut self[p]))
    }

    pub fn set(&mut self, pos: Vec2<isize>, value: T) -> Result<(), String> {
        if let Some(pos) = self.get_mut(pos) {
            *pos = value;
            Ok(())
        } else {
            Err("cannot set value out of bounds !".to_owned())
        }
    }

    pub fn map_to<U: Clone + Default>(self, map_function: fn(T) -> U) -> Matrix<U> {
        let mut new_matrix = Matrix::<U>::new(self.rows, self.cols, U::default());
        for y in 0..self.rows {
            for x in 0..self.cols {
                let pos = Vec2::new(x, y);
                new_matrix[pos] = map_function(self[pos].clone());
            }
        }
        new_matrix
    }

    pub fn contains(&self, pos: Vec2<isize>) -> bool {
        pos.x >= 0 && pos.x < self.cols as isize && pos.y >= 0 && pos.y < self.rows as isize
    }

    pub fn get_rows_uncheked(&self, index: usize) -> Vec<T> {
        (0..self.cols)
            .map(|i| self[Vec2::new(i, index)].clone())
            .collect::<Vec<T>>()
    }

    fn in_range(&self, pos: Vec2<usize>) -> bool {
        pos.x < self.cols && pos.y < self.rows
    }
}

impl<T: PartialEq> Matrix<T> {
    // find the first `el` and return it's coordinates
    pub fn find(&self, el: T) -> Option<Vec2<usize>> {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let coord = Vec2::new(c, r);
                if self[coord] == el {
                    return Some(coord);
                }
            }
        }
        None
    }
}

impl Matrix<char> {
    pub fn from_chars(input: &[String]) -> Self {
        let mut matrix: Matrix<char> = Self::new(input.len(), input[0].len(), ' ');
        for (y, line) in input.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                matrix[Vec2::new(x, y)] = ch;
            }
        }
        matrix
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let cols = if rows > 0 { value[0].len() } else { 0 };
        if value.iter().any(|r| r.len() != cols) {
            panic!("Not from a matrix format");
        }
        let data = value.into_iter().flatten().collect();
        Matrix { rows, cols, data }
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{:?}", self[Vec2::new(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
