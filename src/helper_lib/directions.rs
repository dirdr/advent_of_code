use super::vec2::Vec2;

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
    }

    pub fn counter_clockwise_cycle() -> impl Iterator<Item = Direction> {
        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
    }

    pub fn to_offset(&self) -> Vec2<isize> {
        match self {
            Self::North => Vec2::new(0, -1)
            Self::South => Vec2::new(0, 1),
            Self::East => Vec2::new(1, 0),
            Self::West => Vec2::(-1, 0),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn is_vertical(&self) -> bool {
        *self == Direction::North || *self == Direction::South
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Direction::East || *self == Direction::West
    }
}
