use core::fmt;

use crate::point::Point;

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}

#[derive(Clone, Debug)]
pub struct PointVec2d<T> {
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> PointVec2d<T> {
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

    pub fn index(&self, point: Point) -> &T {
        &self.vec[self.width * point.y + point.x]
    }

    pub fn index_mut(&mut self, point: Point) -> &mut T {
        &mut self.vec[self.width * point.y + point.x]
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
            Direction::East => self.right(point),
            Direction::South => self.down(point),
            Direction::West => self.left(point),
        }
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

    pub fn positions(&self) -> Positions {
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

impl<T> std::ops::Index<Point> for PointVec2d<T> {
    fn index(&self, index: Point) -> &T {
        self.index(index)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<Point> for PointVec2d<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        self.index_mut(index)
    }
}

default impl<T: std::fmt::Debug> std::fmt::Display for PointVec2d<T> {
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

impl std::fmt::Display for PointVec2d<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                str.push_str(if *self.index(Point::new(x, y)) {
                    "■ "
                } else {
                    ". "
                });
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

pub const ADJ_FOUR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

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
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_FOUR[self.current].0;
            let ny = self.y as isize + ADJ_FOUR[self.current].1;
            self.direction = self.direction.rotate_clockwise();

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
                0 => Direction::West,
                1 => Direction::North,
                2 => Direction::South,
                3 => Direction::East,
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
