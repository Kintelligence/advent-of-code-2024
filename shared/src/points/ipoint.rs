use core::fmt;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::{
    point::Point,
    traits::{Absolute, Modulo, ModuloAssign, ModuloPositive, ModuloPositiveAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct IPoint {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for IPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<Point> for IPoint {
    fn from(value: Point) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
}

impl IPoint {
    pub fn new(x: isize, y: isize) -> Self {
        IPoint { x, y }
    }

    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_sub(rhs.x) {
            if let Some(y) = self.y.checked_sub(rhs.y) {
                return Some(IPoint::new(x, y));
            }
        }

        None
    }

    pub fn checked_add(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_add(rhs.x) {
            if let Some(y) = self.y.checked_add(rhs.y) {
                return Some(IPoint::new(x, y));
            }
        }

        None
    }

    pub fn distance_to(&self, rhs: Self) -> usize {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }

    pub fn offset_points(&self, from: isize, to: isize) -> OffsetPointsIter {
        OffsetPointsIter {
            current: 0,
            x: self.x,
            y: self.y,
            inv_offset: from,
            offset: 0,
            range: from,
            last_range: to,
        }
    }
}

pub struct OffsetPointsIter {
    pub x: isize,
    pub y: isize,
    pub range: isize,
    pub last_range: isize,
    pub current: usize,
    pub offset: isize,
    pub inv_offset: isize,
}

impl Iterator for OffsetPointsIter {
    type Item = IPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 4 {
            self.current = 0;
            self.offset += 1;
            self.inv_offset -= 1;
        }

        if self.offset == self.range {
            self.range += 1;
            self.offset = 0;
            self.inv_offset = self.range;
        }

        if self.range == self.last_range {
            return None;
        }

        self.current += 1;
        Some(match self.current {
            1 => IPoint::new(self.x + self.offset, self.y + self.inv_offset),
            2 => IPoint::new(self.x + self.inv_offset, self.y - self.offset),
            3 => IPoint::new(self.x - self.offset, self.y - self.inv_offset),
            4 => IPoint::new(self.x - self.inv_offset, self.y + self.offset),
            _ => panic!("Unreachable code reached"),
        })
    }
}

impl Add for IPoint {
    type Output = IPoint;

    fn add(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<IPoint> for IPoint {
    fn add_assign(&mut self, rhs: IPoint) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
forward_ref_binop!(impl Add, add for IPoint, IPoint);
forward_ref_op_assign!(impl AddAssign, add_assign for IPoint, IPoint);

impl Sub for IPoint {
    type Output = IPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<IPoint> for IPoint {
    fn sub_assign(&mut self, rhs: IPoint) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_binop!(impl Sub, sub for IPoint, IPoint);
forward_ref_op_assign!(impl SubAssign, sub_assign for IPoint, IPoint);

impl Mul<isize> for IPoint {
    type Output = IPoint;

    fn mul(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for IPoint {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

forward_ref_binop!(impl Mul, mul for IPoint, isize);
forward_ref_op_assign!(impl MulAssign, mul_assign for IPoint, isize);

impl Div<isize> for IPoint {
    type Output = IPoint;

    fn div(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<isize> for IPoint {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

forward_ref_binop!(impl Div, div for IPoint, isize);
forward_ref_op_assign!(impl DivAssign, div_assign for IPoint, isize);

impl Modulo<isize> for IPoint {
    type Output = IPoint;

    fn modulo(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl ModuloAssign<isize> for IPoint {
    fn modulo_assign(&mut self, rhs: isize) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

forward_ref_binop!(impl Modulo, modulo for IPoint, isize);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for IPoint, isize);

impl Mul<IPoint> for IPoint {
    type Output = IPoint;

    fn mul(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<IPoint> for IPoint {
    fn mul_assign(&mut self, rhs: IPoint) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

forward_ref_binop!(impl Mul, mul for IPoint, IPoint);
forward_ref_op_assign!(impl MulAssign, mul_assign for IPoint, IPoint);

impl Div<IPoint> for IPoint {
    type Output = IPoint;

    fn div(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<IPoint> for IPoint {
    fn div_assign(&mut self, rhs: IPoint) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

forward_ref_binop!(impl Div, div for IPoint, IPoint);
forward_ref_op_assign!(impl DivAssign, div_assign for IPoint, IPoint);

impl Modulo<IPoint> for IPoint {
    type Output = IPoint;

    fn modulo(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl ModuloAssign<IPoint> for IPoint {
    fn modulo_assign(&mut self, rhs: IPoint) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

forward_ref_binop!(impl Modulo, modulo for IPoint, IPoint);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for IPoint, IPoint);

impl ModuloPositive<IPoint> for IPoint {
    type Output = IPoint;

    fn modulo_positive(self, rhs: IPoint) -> Self::Output {
        let mut x = self.x % rhs.x;
        let mut y = self.y % rhs.y;

        if x < 0 {
            x += rhs.x;
        }

        if y < 0 {
            y += rhs.y;
        }

        IPoint { x, y }
    }
}

impl ModuloPositiveAssign<IPoint> for IPoint {
    fn module_positive_assign(&mut self, rhs: IPoint) {
        self.x %= rhs.x;
        self.y %= rhs.y;

        if self.x < 0 {
            self.x += rhs.x;
        }

        if self.y < 0 {
            self.y += rhs.y;
        }
    }
}

forward_ref_binop!(impl ModuloPositive, modulo_positive for IPoint, IPoint);
forward_ref_op_assign!(impl ModuloPositiveAssign, module_positive_assign for IPoint, IPoint);

impl ModuloPositive<isize> for IPoint {
    type Output = IPoint;

    fn modulo_positive(self, rhs: isize) -> Self::Output {
        let mut x = self.x % rhs;
        let mut y = self.y % rhs;

        if x < 0 {
            x += rhs;
        }

        if y < 0 {
            y += rhs;
        }

        IPoint { x, y }
    }
}

impl ModuloPositiveAssign<isize> for IPoint {
    fn module_positive_assign(&mut self, rhs: isize) {
        self.x %= rhs;
        self.y %= rhs;

        if self.x < 0 {
            self.x += rhs;
        }

        if self.y < 0 {
            self.y += rhs;
        }
    }
}

forward_ref_binop!(impl ModuloPositive, modulo_positive for IPoint, isize);
forward_ref_op_assign!(impl ModuloPositiveAssign, module_positive_assign for IPoint, isize);

impl Absolute for IPoint {
    fn absolute(self) -> Self {
        IPoint {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Sum for IPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(IPoint::new(0, 0), |acc, item| acc + item)
    }
}
