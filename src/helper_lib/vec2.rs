use std::fmt::Debug;
use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

pub struct ConversionError;

impl Debug for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Conversion cannot be done !")
    }
}

impl TryFrom<Vec2<isize>> for Vec2<usize> {
    type Error = ConversionError;

    fn try_from(value: Vec2<isize>) -> Result<Self, Self::Error> {
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
