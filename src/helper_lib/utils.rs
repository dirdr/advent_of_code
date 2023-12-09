use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row * self.cols + col]
    }
}

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

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
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
