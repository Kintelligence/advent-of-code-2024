use iterators::*;

use crate::points::{directions::Direction, point::Point};

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
    pub fn filled(fill: T, height: usize, width: usize) -> Self {
        Self {
            vec: vec![fill; height * width],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn from(vec: Vec<T>, height: usize) -> Self {
        Self {
            width: vec.len() / height,
            vec,
            height,
        }
    }

    pub fn same_size_with<TNew>(&self, fill: TNew) -> Grid<TNew>
    where
        TNew: Clone,
    {
        Grid {
            vec: vec![fill; self.height * self.width],
            width: self.width,
            height: self.height,
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

    pub fn adjacent_three_in_direction(
        &self,
        point: Point,
        from: Direction,
    ) -> AdjacentThreeInDirection {
        AdjacentThreeInDirection {
            x: point.x,
            y: point.y,
            height: self.height,
            width: self.width,
            current: 0,
            direction: from.reverse(),
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

impl Grid<usize> {
    pub fn print_usize(&self, unit_width: usize) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                str.push_str(&format!("{:width$?} ", &self[(x, y)], width = unit_width,));
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!(
                "{:<width$} ",
                x % (10usize.pow(unit_width as u32)),
                width = unit_width,
            ));
        }

        str.push('\n');
        str
    }
}

impl Grid<u8> {
    pub fn print_u8(&self, unit_width: usize) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                str.push_str(&format!("{:width$?} ", &self[(x, y)], width = unit_width,));
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!(
                "{:<width$} ",
                x % (10usize.pow(unit_width as u32)),
                width = unit_width,
            ));
        }

        str.push('\n');
        str
    }
}

impl Grid<Option<usize>> {
    pub fn print_option_usize(&self, unit_width: usize) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                if let Some(value) = &self[(x, y)] {
                    str.push_str(&format!("{:<width$} ", value, width = unit_width,));
                } else {
                    str.push_str(&format!("{:width$} ", '.', width = unit_width,));
                }
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!(
                "{:<width$} ",
                x % (10usize.pow(unit_width as u32)),
                width = unit_width,
            ));
        }

        str.push('\n');
        str
    }
}

impl Grid<Option<u8>> {
    pub fn print_option_u8(&self, unit_width: usize) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                if let Some(value) = &self[(x, y)] {
                    str.push_str(&format!("{:<width$} ", value, width = unit_width,));
                } else {
                    str.push_str(&format!("{:width$} ", '.', width = unit_width,));
                }
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!(
                "{:<width$} ",
                x % (10usize.pow(unit_width as u32)),
                width = unit_width,
            ));
        }

        str.push('\n');
        str
    }
}

impl Grid<bool> {
    pub fn print_bool(&self) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                str.push_str(if self[Point::new(x, y)] { "■ " } else { ". " });
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!("{:} ", x % 10));
        }

        str.push('\n');
        str
    }

    pub fn go_if_true(&self, point: Point, direction: Direction) -> Option<Point> {
        (match direction {
            Direction::North => self.up(point),
            Direction::NorthEast => self.up_right(point),
            Direction::East => self.right(point),
            Direction::SouthEast => self.down_right(point),
            Direction::South => self.down(point),
            Direction::SouthWest => self.down_left(point),
            Direction::West => self.left(point),
            Direction::NorthWest => self.up_left(point),
        })
        .and_then(|p| if self[p] { Some(p) } else { None })
    }
}

impl<T> Grid<T>
where
    T: std::fmt::Debug,
{
    pub fn print_debug(&self, unit_width: usize) -> String {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                str.push_str(&format!("{:width$?} ", &self[(x, y)], width = unit_width,));
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!(
                "{:<width$} ",
                x % (10usize.pow(unit_width as u32)),
                width = unit_width,
            ));
        }

        str.push('\n');
        str
    }
}

pub mod iterators;
