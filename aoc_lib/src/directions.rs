use super::vec2::Vec2;

pub trait Direction {
    fn all_clockwise() -> impl Iterator<Item = Self>;
    fn all_counter_clockwise() -> impl Iterator<Item = Self>;

    fn opposite(self) -> Self;
    fn turn_left(self) -> Self;
    fn turn_right(self) -> Self;

    fn to_offset(self) -> Vec2<isize>;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ExtendedCardinal {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    East,
    West,
}

impl Direction for Cardinal {
    fn all_clockwise() -> impl Iterator<Item = Self> {
        vec![Self::North, Self::East, Self::South, Self::West].into_iter()
    }

    fn all_counter_clockwise() -> impl Iterator<Item = Self> {
        vec![Self::North, Self::West, Self::South, Self::East].into_iter()
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn to_offset(self) -> Vec2<isize> {
        match self {
            Self::North => Vec2::new(0, -1),
            Self::South => Vec2::new(0, 1),
            Self::East => Vec2::new(1, 0),
            Self::West => Vec2::new(-1, 0),
        }
    }
}

impl Direction for ExtendedCardinal {
    fn all_clockwise() -> impl Iterator<Item = Self> {
        vec![
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
        .into_iter()
    }

    fn all_counter_clockwise() -> impl Iterator<Item = Self> {
        vec![
            Self::North,
            Self::NorthWest,
            Self::West,
            Self::SouthWest,
            Self::South,
            Self::SouthEast,
            Self::East,
            Self::NorthEast,
        ]
        .into_iter()
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthWest => Self::SouthEast,
            Self::NorthEast => Self::SouthWest,
            Self::South => Self::North,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::NorthWest => Self::West,
            Self::West => Self::SouthWest,
            Self::SouthWest => Self::South,
            Self::South => Self::SouthEast,
            Self::SouthEast => Self::East,
            Self::East => Self::NorthEast,
            Self::NorthEast => Self::North,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }
    fn to_offset(self) -> Vec2<isize> {
        match self {
            Self::North => Vec2::new(0, -1),
            Self::NorthWest => Vec2::new(-1, -1),
            Self::NorthEast => Vec2::new(1, -1),
            Self::South => Vec2::new(0, 1),
            Self::SouthWest => Vec2::new(-1, 1),
            Self::SouthEast => Vec2::new(1, 1),
            Self::East => Vec2::new(1, 0),
            Self::West => Vec2::new(-1, 0),
        }
    }
}
