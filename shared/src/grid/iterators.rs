use crate::points::{directions::Direction, point::Point};

pub const DIAGONAL: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

pub struct Diagonals {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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

pub struct Positions {
    pub height: usize,
    pub width: usize,
    pub x: usize,
    pub y: usize,
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
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
    pub direction: Direction,
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
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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
