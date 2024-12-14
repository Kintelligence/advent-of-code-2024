use core::fmt;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use crate::{
    common::{Modulo, ModuloAssign},
    ipoint::IPoint,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn from(ipoint: IPoint) -> Option<Self> {
        if ipoint.x >= 0 && ipoint.y >= 0 {
            return Some(Point::new(ipoint.x as usize, ipoint.y as usize));
        }
        None
    }

    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_sub(rhs.x) {
            if let Some(y) = self.y.checked_sub(rhs.y) {
                return Some(Point::new(x, y));
            }
        }

        None
    }

    pub fn checked_add(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_add(rhs.x) {
            if let Some(y) = self.y.checked_add(rhs.y) {
                return Some(Point::new(x, y));
            }
        }

        None
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
forward_ref_binop!(impl Add, add for Point, Point);
forward_ref_op_assign!(impl AddAssign, add_assign for Point, Point);

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_binop!(impl Sub, sub for Point, Point);
forward_ref_op_assign!(impl SubAssign, sub_assign for Point, Point);

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<usize> for Point {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

forward_ref_binop!(impl Mul, mul for Point, usize);
forward_ref_op_assign!(impl MulAssign, mul_assign for Point, usize);

impl Div<usize> for Point {
    type Output = Point;

    fn div(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<usize> for Point {
    fn div_assign(&mut self, rhs: usize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

forward_ref_binop!(impl Div, div for Point, usize);
forward_ref_op_assign!(impl DivAssign, div_assign for Point, usize);

impl Modulo<usize> for Point {
    type Output = Point;

    fn modulo(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl ModuloAssign<usize> for Point {
    fn modulo_assign(&mut self, rhs: usize) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

forward_ref_binop!(impl Modulo, modulo for Point, usize);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Point, usize);

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, rhs: Point) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

forward_ref_binop!(impl Mul, mul for Point, Point);
forward_ref_op_assign!(impl MulAssign, mul_assign for Point, Point);

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, rhs: Point) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

forward_ref_binop!(impl Div, div for Point, Point);
forward_ref_op_assign!(impl DivAssign, div_assign for Point, Point);

impl Modulo<Point> for Point {
    type Output = Point;

    fn modulo(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl ModuloAssign<Point> for Point {
    fn modulo_assign(&mut self, rhs: Point) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

forward_ref_binop!(impl Modulo, modulo for Point, Point);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Point, Point);

impl Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Point::new(0, 0), |acc, item| acc + item)
    }
}
