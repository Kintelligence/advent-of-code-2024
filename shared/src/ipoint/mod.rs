use core::fmt;
use std::ops::{Add, Sub};

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
}

impl Add for &IPoint {
    type Output = IPoint;

    fn add(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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

impl Add<IPoint> for &IPoint {
    type Output = IPoint;

    fn add(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&IPoint> for IPoint {
    type Output = IPoint;

    fn add(self, rhs: &IPoint) -> Self::Output {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for IPoint {
    type Output = IPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &IPoint {
    type Output = IPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<IPoint> for &IPoint {
    type Output = IPoint;

    fn sub(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&IPoint> for IPoint {
    type Output = IPoint;

    fn sub(self, rhs: &IPoint) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
