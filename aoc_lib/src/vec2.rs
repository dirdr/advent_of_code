use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Sub};

use num::{Num, Signed};
use std::hash::Hash;

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Signed + Copy> Vec2<T> {
    pub fn abs(&self) -> Vec2<T> {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Add<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, scalar: T) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> AddAssign<T> for Vec2<T>
where
    T: Num + Copy,
{
    fn add_assign(&mut self, scalar: T) {
        self.x = self.x + scalar;
        self.y = self.y + scalar;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, scalar: T) -> Vec2<T> {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub struct ConversionError;

impl Debug for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conversion cannot be done !")
    }
}

// TODO maybe tryfrom and from generalized over num traits
impl TryFrom<Vec2<isize>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: Vec2<isize>) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x).map_err(|_| ConversionError)?;
        let y = usize::try_from(value.y).map_err(|_| ConversionError)?;
        Ok(Vec2::new(x, y))
    }
}

impl TryFrom<&Vec2<isize>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: &Vec2<isize>) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x).map_err(|_| ConversionError)?;
        let y = usize::try_from(value.y).map_err(|_| ConversionError)?;
        Ok(Vec2::new(x, y))
    }
}

impl From<Vec2<usize>> for Vec2<isize> {
    fn from(value: Vec2<usize>) -> Self {
        Vec2::new(value.x as isize, value.y as isize)
    }
}

impl From<&Vec2<usize>> for Vec2<isize> {
    fn from(value: &Vec2<usize>) -> Self {
        Vec2::new(value.x as isize, value.y as isize)
    }
}

impl From<Vec2<usize>> for Vec2<f64> {
    fn from(value: Vec2<usize>) -> Self {
        Vec2::new(value.x as f64, value.y as f64)
    }
}

impl<T: Num> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Num + Hash> Hash for Vec2<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Num> Eq for Vec2<T> {}
