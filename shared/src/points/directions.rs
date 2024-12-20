use core::fmt;

pub const CARDINALS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const DIRECTIONS: [Direction; 8] = [
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

pub const CARDINALS_EXCEPT_NORTH: [Direction; 3] =
    [Direction::East, Direction::South, Direction::West];

pub const CARDINALS_EXCEPT_EAST: [Direction; 3] =
    [Direction::North, Direction::South, Direction::West];

pub const CARDINALS_EXCEPT_SOUTH: [Direction; 3] =
    [Direction::North, Direction::East, Direction::West];

pub const CARDINALS_EXCEPT_WEST: [Direction; 3] =
    [Direction::North, Direction::East, Direction::South];

impl Direction {
    pub fn other_cardinals(&self) -> [Direction; 3] {
        match self {
            Direction::North => CARDINALS_EXCEPT_NORTH,
            Direction::East => CARDINALS_EXCEPT_EAST,
            Direction::South => CARDINALS_EXCEPT_SOUTH,
            Direction::West => CARDINALS_EXCEPT_WEST,
            _ => panic!("Not allowed on ordinals"),
        }
    }

    pub fn rotate_90(&self) -> Self {
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

    pub fn rotate_counter_90(&self) -> Self {
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

    pub fn rotate_45(&self) -> Self {
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

    pub fn rotate_counter_45(&self) -> Self {
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

    pub fn is_vertical(&self) -> bool {
        return *self == Direction::North || *self == Direction::South;
    }

    pub fn is_horizontal(&self) -> bool {
        return *self == Direction::East || *self == Direction::West;
    }

    pub fn is_cardinal(&self) -> bool {
        return *self == Direction::East
            || *self == Direction::West
            || *self == Direction::North
            || *self == Direction::South;
    }

    pub fn is_ordinal(&self) -> bool {
        return *self == Direction::NorthEast
            || *self == Direction::SouthEast
            || *self == Direction::SouthWest
            || *self == Direction::NorthWest;
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}
