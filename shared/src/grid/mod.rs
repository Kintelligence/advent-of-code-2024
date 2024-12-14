use core::fmt;

use crate::point::Point;

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

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn empty(fill: T, height: usize, width: usize) -> Self {
        Self {
            vec: vec![fill; height * width],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn from_vec(vec: Vec<T>, height: usize) -> Self {
        Self {
            width: vec.len() / height,
            vec,
            height,
        }
    }

    pub fn new(vec: Vec<T>, width: usize, height: usize) -> Self {
        Self { vec, width, height }
    }

    pub fn overwrite(&mut self, vec: Vec<T>) -> bool {
        if self.vec.len() == vec.len() {
            self.vec = vec;
            return true;
        }
        false
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.vec[self.width * y..(self.width * y + self.width)]
    }

    fn index_xy(&self, x: usize, y: usize) -> &T {
        &self.vec[self.width * y + x]
    }

    fn index_xy_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[self.width * y + x]
    }

    pub fn is_within_bounds(&mut self, point: Point) -> bool {
        return point.x < self.width && point.y < self.height;
    }

    pub fn insert(&mut self, point: Point, value: T) -> bool {
        if self.is_within_bounds(point) {
            self.vec[self.width * point.y + point.x] = value;

            return true;
        }
        false
    }

    pub fn go(&self, point: Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::North => self.up(point),
            Direction::NorthEast => self.up_right(point),
            Direction::East => self.right(point),
            Direction::SouthEast => self.down_right(point),
            Direction::South => self.down(point),
            Direction::SouthWest => self.down_left(point),
            Direction::West => self.left(point),
            Direction::NorthWest => self.up_left(point),
        }
    }

    pub fn go_xy(&self, x: usize, y: usize, direction: Direction) -> Option<Point> {
        match direction {
            Direction::North => self.up_xy(x, y),
            Direction::NorthEast => self.up_right_xy(x, y),
            Direction::East => self.right_xy(x, y),
            Direction::SouthEast => self.down_right_xy(x, y),
            Direction::South => self.down_xy(x, y),
            Direction::SouthWest => self.down_left_xy(x, y),
            Direction::West => self.left_xy(x, y),
            Direction::NorthWest => self.up_left_xy(x, y),
        }
    }

    pub fn up_left(&self, point: Point) -> Option<Point> {
        if point.y == 0 || point.x == 0 {
            return None;
        }

        Some(Point::new(point.x - 1, point.y - 1))
    }

    pub fn up_right(&self, point: Point) -> Option<Point> {
        if point.y == 0 || point.x >= self.width - 1 {
            return None;
        }

        Some(Point::new(point.x + 1, point.y - 1))
    }

    pub fn down_left(&self, point: Point) -> Option<Point> {
        if point.y >= self.height - 1 || point.x == 0 {
            return None;
        }

        Some(Point::new(point.x - 1, point.y + 1))
    }

    pub fn down_right(&self, point: Point) -> Option<Point> {
        if point.y >= self.height - 1 || point.x >= self.width - 1 {
            return None;
        }

        Some(Point::new(point.x + 1, point.y + 1))
    }

    pub fn up(&self, point: Point) -> Option<Point> {
        if point.y == 0 {
            return None;
        }

        Some(Point::new(point.x, point.y - 1))
    }

    pub fn down(&self, point: Point) -> Option<Point> {
        if point.y >= self.height - 1 {
            return None;
        }

        Some(Point::new(point.x, point.y + 1))
    }

    pub fn right(&self, point: Point) -> Option<Point> {
        if point.x >= self.width - 1 {
            return None;
        }

        Some(Point::new(point.x + 1, point.y))
    }

    pub fn left(&self, point: Point) -> Option<Point> {
        if point.x == 0 {
            return None;
        }

        Some(Point::new(point.x - 1, point.y))
    }

    pub fn up_left_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y == 0 || x == 0 {
            return None;
        }

        Some(Point::new(x - 1, y - 1))
    }

    pub fn up_right_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y == 0 || x >= self.width - 1 {
            return None;
        }

        Some(Point::new(x + 1, y - 1))
    }

    pub fn down_left_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y >= self.height - 1 || x == 0 {
            return None;
        }

        Some(Point::new(x - 1, y + 1))
    }

    pub fn down_right_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y >= self.height - 1 || x >= self.width - 1 {
            return None;
        }

        Some(Point::new(x + 1, y + 1))
    }

    pub fn up_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y == 0 {
            return None;
        }

        Some(Point::new(x, y - 1))
    }

    pub fn down_xy(&self, x: usize, y: usize) -> Option<Point> {
        if y >= self.height - 1 {
            return None;
        }

        Some(Point::new(x, y + 1))
    }

    pub fn right_xy(&self, x: usize, y: usize) -> Option<Point> {
        if x >= self.width - 1 {
            return None;
        }

        Some(Point::new(x + 1, y))
    }

    pub fn left_xy(&self, x: usize, y: usize) -> Option<Point> {
        if x == 0 {
            return None;
        }

        Some(Point::new(x - 1, y))
    }

    pub fn points(&self) -> Positions {
        Positions {
            height: self.height,
            width: self.width,
            x: 0,
            y: 0,
        }
    }

    pub fn adjacent_eight(&self, point: Point) -> AdjecentEight {
        AdjecentEight {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn adjacent_four(&self, point: Point) -> AdjacentFour {
        AdjacentFour {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn adjacent_four_directional(&self, point: Point) -> AdjacentFourDirectional {
        AdjacentFourDirectional {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn adjacent_three_directional(
        &self,
        point: Point,
        from: Direction,
    ) -> AdjacentThreeDirectional {
        AdjacentThreeDirectional {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
            direction: from,
        }
    }

    pub fn diagonals(&self, point: Point) -> Diagonals {
        Diagonals {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    fn index(&self, index: Point) -> &T {
        self.index_xy(index.x, index.y)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        self.index_xy_mut(index.x, index.y)
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    fn index(&self, index: (usize, usize)) -> &T {
        self.index_xy(index.0, index.1)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.index_xy_mut(index.0, index.1)
    }
}

default impl<T: std::fmt::Debug> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            str.push_str(&format!("{:?}", &self.row(i)));
            if i != self.height - 1 {
                str.push_str(", ");
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                str.push_str(format!("{:2} ", self[(x, y)]).as_str());
            }

            str.push('\n');
        }
        str.push('\n');
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                str.push_str(if self[Point::new(x, y)] { "■ " } else { ". " });
            }

            str.push('\n');
        }
        str.push('\n');
        write!(f, "{}", str)
    }
}

pub struct Positions {
    height: usize,
    width: usize,
    x: usize,
    y: usize,
}

impl Iterator for Positions {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        let point = Point::new(self.x, self.y);
        self.x += 1;

        return Some(point);
    }
}

pub const ADJ_EIGHT: [(isize, isize); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

pub struct AdjecentEight {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for AdjecentEight {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 7 {
                return None;
            }

            let nx = self.x as isize + ADJ_EIGHT[self.current].0;
            let ny = self.y as isize + ADJ_EIGHT[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some(Point::new(nx as usize, ny as usize));
        }
    }
}

pub const ADJ_FOUR: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct AdjacentFour {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for AdjacentFour {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_FOUR[self.current].0;
            let ny = self.y as isize + ADJ_FOUR[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some(Point::new(nx as usize, ny as usize));
        }
    }
}

pub struct AdjacentThreeDirectional {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
    direction: Direction,
}

impl Iterator for AdjacentThreeDirectional {
    type Item = (Point, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 2 {
                return None;
            }

            self.direction = self.direction.rotate_clockwise_90();
            let nx = self.x as isize + ADJ_FOUR[self.direction as usize].0;
            let ny = self.y as isize + ADJ_FOUR[self.direction as usize].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((Point::new(nx as usize, ny as usize), self.direction));
        }
    }
}

pub struct AdjacentFourDirectional {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for AdjacentFourDirectional {
    type Item = (Point, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_FOUR[self.current].0;
            let ny = self.y as isize + ADJ_FOUR[self.current].1;
            let dir = match self.current {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => panic!(),
            };

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((Point::new(nx as usize, ny as usize), dir));
        }
    }
}

pub const DIAGONAL: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

pub struct Diagonals {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for Diagonals {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + DIAGONAL[self.current].0;
            let ny = self.y as isize + DIAGONAL[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some(Point::new(nx as usize, ny as usize));
        }
    }
}
