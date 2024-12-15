use crate::parse::Parsable;

use super::{ipoint::IPoint, point::Point};

impl<T: Iterator<Item = u8>> Parsable<Point> for T {
    fn next_number(&mut self) -> Option<Point> {
        if let Some((x, y)) = self.next_number().zip(self.next_number()) {
            return Some(Point::new(x, y));
        }
        None
    }
}

impl<T: Iterator<Item = u8>> Parsable<IPoint> for T {
    fn next_number(&mut self) -> Option<IPoint> {
        if let Some((x, y)) = self.next_number().zip(self.next_number()) {
            return Some(IPoint::new(x, y));
        }
        None
    }
}
