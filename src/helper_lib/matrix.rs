use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (col, row) = index;
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (col, row) = index;
        &mut self.data[row * self.cols + col]
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

    pub fn get(&self, row: isize, col: isize) -> Option<&T> {
        if row < 0 || col < 0 {
            return None;
        }
        if row as usize >= self.rows || col as usize >= self.cols {
            return None;
        }
        Some(&self.data[row as usize * self.cols + col as usize])
    }

    pub fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut T> {
        if row < 0 || col < 0 {
            return None;
        }
        if row as usize >= self.rows || col as usize >= self.cols {
            return None;
        }
        Some(&mut self.data[row as usize * self.cols + col as usize])
    }

    pub fn set(&mut self, row: isize, col: isize, value: T) -> Result<(), String> {
        if row < 0 || col < 0 || row as usize >= self.rows || col as usize >= self.cols {
            return Err("cannot set value out of bounds".to_string());
        }
        self.data[row as usize * self.cols + col as usize] = value;
        Ok(())
    }
}

impl Matrix<char> {
    pub fn from_chars(input: &[String]) -> Self {
        let mut matrix: Matrix<char> = Self::new(input.len(), input[0].len(), ' ');
        for (y, line) in input.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                matrix[(y, x)] = ch;
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
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:?}", self[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
