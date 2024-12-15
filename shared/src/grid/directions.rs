use core::fmt;

pub const DIRECTIONS4: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const DIRECTIONS8: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    NorthEast = 4,
    SouthEast = 5,
    SouthWest = 6,
    NorthWest = 7,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::East => "E",
                Direction::South => "S",
                Direction::West => "W",
                Direction::NorthEast => "NE",
                Direction::SouthEast => "SE",
                Direction::SouthWest => "SW",
                Direction::NorthWest => "NW",
            }
        )
    }
}

impl Direction {
    pub fn rotate_clockwise_90(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
            Direction::NorthEast => Self::SouthEast,
            Direction::SouthEast => Self::SouthWest,
            Direction::SouthWest => Self::NorthWest,
            Direction::NorthWest => Self::NorthEast,
        }
    }

    pub fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
            Direction::NorthEast => Self::NorthWest,
            Direction::SouthEast => Self::NorthEast,
            Direction::SouthWest => Self::SouthEast,
            Direction::NorthWest => Self::SouthWest,
        }
    }

    pub fn rotate_clockwise_45(&self) -> Self {
        match self {
            Direction::North => Self::NorthEast,
            Direction::NorthEast => Self::East,
            Direction::East => Self::SouthEast,
            Direction::SouthEast => Self::South,
            Direction::South => Self::SouthWest,
            Direction::SouthWest => Self::West,
            Direction::West => Self::NorthWest,
            Direction::NorthWest => Self::North,
        }
    }

    pub fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            Direction::North => Self::NorthWest,
            Direction::NorthEast => Self::North,
            Direction::East => Self::NorthEast,
            Direction::SouthEast => Self::East,
            Direction::South => Self::SouthEast,
            Direction::SouthWest => Self::South,
            Direction::West => Self::SouthWest,
            Direction::NorthWest => Self::West,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
            Direction::NorthEast => Self::SouthWest,
            Direction::SouthEast => Self::NorthWest,
            Direction::SouthWest => Self::NorthEast,
            Direction::NorthWest => Self::SouthEast,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}
