use num::Num;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vec3<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
