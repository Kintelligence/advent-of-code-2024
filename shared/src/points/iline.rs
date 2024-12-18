use std::fmt;

use super::ipoint::IPoint;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ILine {
    from: IPoint,
    to: IPoint,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Intersection {
    Point(IPoint),
    Segment(ILine),
    None,
}

impl std::fmt::Display for ILine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}->{}]", self.from, self.to)
    }
}

impl ILine {
    pub fn new(a: IPoint, b: IPoint) -> Self {
        let from = a.min(b);
        let to = a.max(b);
        ILine { from, to }
    }

    pub fn len(&self) -> usize {
        self.from.distance_to(self.to)
    }

    pub fn contains(&self, point: IPoint) -> bool {
        (self.to.x - self.from.x) * (point.y - self.from.y)
            - (self.to.y - self.from.y) * (point.x - self.from.x)
            == 0
    }

    pub fn points(&self) -> Vec<IPoint> {
        let dx = self.to.x.abs_diff(self.from.x) as isize;
        let sx = if self.from.x < self.to.x { 1 } else { -1 };
        let dy = -(self.to.y.abs_diff(self.from.y) as isize);
        let sy = if self.from.y < self.to.y { 1 } else { -1 };
        let mut error = dx + dy;

        let mut points = Vec::new();

        let mut x = self.from.x;
        let mut y = self.from.y;

        loop {
            points.push(IPoint::new(x, y));
            if x == self.to.x && y == self.to.y {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                error = error + dy;
                x += sx;
            }

            if e2 <= dx {
                error = error + dx;
                y += sy;
            }
        }

        points
    }

    pub fn intersect(&self, other: &ILine) -> Intersection {
        // Compute determinants to check relative orientation
        let d1 = (self.to.x - self.from.x) * (other.from.y - self.from.y)
            - (self.to.y - self.from.y) * (other.from.x - self.from.x);
        let d2 = (self.to.x - self.from.x) * (other.to.y - self.from.y)
            - (self.to.y - self.from.y) * (other.to.x - self.from.x);

        let d3 = (other.to.x - other.from.x) * (self.from.y - other.from.y)
            - (other.to.y - other.from.y) * (self.from.x - other.from.x);
        let d4 = (other.to.x - other.from.x) * (self.to.y - other.from.y)
            - (other.to.y - other.from.y) * (self.to.x - other.from.x);

        // Case 1: Proper intersection
        if d1 * d2 < 0 && d3 * d4 < 0 {
            // Compute the intersection point
            let a1 = self.to.y - self.from.y;
            let b1 = self.from.x - self.to.x;
            let c1 = a1 * self.from.x + b1 * self.from.y;

            let a2 = other.to.y - other.from.y;
            let b2 = other.from.x - other.to.x;
            let c2 = a2 * other.from.x + b2 * other.from.y;

            let determinant = a1 * b2 - a2 * b1;

            if determinant == 0 {
                return Intersection::None; // Parallel lines
            }

            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;

            return Intersection::Point(IPoint::new(x, y));
        }

        // Case 2: Collinear lines
        if d1 == 0 && d2 == 0 && d3 == 0 && d4 == 0 {
            // Compute the overlapping range using bounding boxes
            let overlap_start_x = self.from.x.max(other.from.x);
            let overlap_end_x = self.to.x.min(other.to.x);
            let overlap_start_y = self.from.y.max(other.from.y);
            let overlap_end_y = self.to.y.min(other.to.y);

            if overlap_start_x > overlap_end_x || overlap_start_y > overlap_end_y {
                return Intersection::None; // No overlap
            }

            if overlap_start_x == overlap_end_x && overlap_start_y == overlap_end_y {
                return Intersection::Point(IPoint::new(overlap_start_x, overlap_start_y));
            }

            return Intersection::Segment(ILine::new(
                IPoint::new(overlap_start_x, overlap_start_y),
                IPoint::new(overlap_end_x, overlap_end_y),
            ));
        }

        // Case 3: Endpoint touching or one endpoint lies on the other segment
        if self.contains(other.from) {
            return Intersection::Point(other.from);
        }
        if self.contains(other.to) {
            return Intersection::Point(other.to);
        }
        if other.contains(self.from) {
            return Intersection::Point(self.from);
        }
        if other.contains(self.to) {
            return Intersection::Point(self.to);
        }

        Intersection::None
    }
}

#[cfg(test)]
mod iline_tests {
    use super::ILine;
    use crate::*;
    use points::{iline::Intersection, ipoint::IPoint};
    use test_case::test_case;

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(1, 2)),
        ILine::new(IPoint::new(0, 1), IPoint::new(2, 1)),
        IPoint::new(1, 1)
    )]
    #[test_case(
        ILine::new(IPoint::new(0, 0), IPoint::new(0, 1)),
        ILine::new(IPoint::new(0, 0), IPoint::new(1, 0)),
        IPoint::new(0, 0)
    )]
    fn intersection_point_test(a: ILine, b: ILine, expected: IPoint) {
        assert_eq!(a.intersect(&b), Intersection::Point(expected));
    }

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(3, 0)),
        ILine::new(IPoint::new(0, 0), IPoint::new(2, 0)),
        ILine::new(IPoint::new(1, 0), IPoint::new(2, 0))
    )]
    fn intersection_line_test(a: ILine, b: ILine, expected: ILine) {
        assert_eq!(a.intersect(&b), Intersection::Segment(expected));
    }

    #[test_case(
        ILine::new(IPoint::new(1, 0), IPoint::new(1, 2)),
        ILine::new(IPoint::new(0, 0), IPoint::new(0, 2))
    )]
    fn intersection_none_test(a: ILine, b: ILine) {
        assert_eq!(a.intersect(&b), Intersection::None);
    }

    #[test_case(
        ILine::new(IPoint::new(2, 6), IPoint::new(8, 6)),
        ILine::new(IPoint::new(4, 1), IPoint::new(4, 6)),
        IPoint::new(4, 6)
    )]
    fn intersection_endpoint_test(a: ILine, b: ILine, point: IPoint) {
        assert_eq!(a.intersect(&b), Intersection::Point(point));
    }
}
