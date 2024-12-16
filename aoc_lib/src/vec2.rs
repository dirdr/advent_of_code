use std::fmt::{self, Debug};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::{Num, Signed, ToPrimitive};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub struct ConversionError;

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vec2<T>
where
    T: Signed + Copy,
{
    pub fn abs(&self) -> Vec2<T> {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Num + Add<Output = T>,
{
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

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign + Num,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> AddAssign<T> for Vec2<T>
where
    T: Num + Copy + AddAssign,
{
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl<T> Sub for Vec2<T>
where
    T: Num + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: SubAssign + Num,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> SubAssign<T> for Vec2<T>
where
    T: Num + Copy + SubAssign,
{
    fn sub_assign(&mut self, scalar: T) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Vec2<T> {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> MulAssign<T> for Vec2<T>
where
    T: Num + Copy + MulAssign,
{
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Num + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        if scalar.is_zero() {
            panic!("Cannot divide Vec2 by zero");
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T> DivAssign<T> for Vec2<T>
where
    T: Num + Copy + DivAssign,
{
    fn div_assign(&mut self, scalar: T) {
        if scalar.is_zero() {
            panic!("Cannot divide Vec2 by zero");
        }
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl TryFrom<Vec2<isize>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: Vec2<isize>) -> Result<Self, Self::Error> {
        let x = value.x.to_usize().ok_or(ConversionError)?;
        let y = value.y.to_usize().ok_or(ConversionError)?;
        Ok(Vec2::new(x, y))
    }
}

impl TryFrom<&Vec2<isize>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: &Vec2<isize>) -> Result<Self, Self::Error> {
        let x = value.x.to_usize().ok_or(ConversionError)?;
        let y = value.y.to_usize().ok_or(ConversionError)?;
        Ok(Vec2::new(x, y))
    }
}

impl TryFrom<Vec2<i32>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: Vec2<i32>) -> Result<Self, Self::Error> {
        let x = value.x.to_usize().ok_or(ConversionError)?;
        let y = value.y.to_usize().ok_or(ConversionError)?;
        Ok(Vec2::new(x, y))
    }
}

impl TryFrom<&Vec2<i32>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: &Vec2<i32>) -> Result<Self, Self::Error> {
        let x = value.x.to_usize().ok_or(ConversionError)?;
        let y = value.y.to_usize().ok_or(ConversionError)?;
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

impl Vec2<isize> {
    pub fn to_usize_unchecked(self) -> Vec2<usize> {
        Vec2::new(self.x as usize, self.y as usize)
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conversion cannot be done !")
    }
}
