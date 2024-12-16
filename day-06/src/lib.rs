use std::{
    fmt::{self},
    usize,
};

use grid::Grid;
use points::{directions::Direction, point::Point};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dest {
    Turn(Point),
    Leave,
    None,
}

struct DirectionGrid {
    grid: PointGrid<[Dest; 4]>,
}

impl std::fmt::Display for DirectionGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for dir in 0..4 {
            match dir {
                0 => str.push_str("North"),
                1 => str.push_str("East"),
                2 => str.push_str("South"),
                3 => str.push_str("West"),
                _ => {}
            }

            str.push('\n');
            for y in 0..self.grid.height {
                for x in 0..self.grid.width {
                    let b = self.grid[(x, y)][dir];
                    match b {
                        Dest::None => str.push_str("   ■    "),
                        Dest::Turn(point) => {
                            str.push_str(format!("{:3},{:<3} ", point.x, point.y).as_str())
                        }
                        Dest::Leave => str.push_str("   x    "),
                    };
                }

                str.push('\n');
            }
            str.push('\n');
        }

        write!(f, "{}", str)
    }
}

fn parse(input: &str) -> (DirectionGrid, Point) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut grid = PointGrid::empty([Dest::None; 4], height, width);

    let mut start = Point::new(0, 0);
    let mut above_points: Vec<Point> = (0..width).into_iter().map(|i| Point::new(i, 0)).collect();
    let mut above_dests = vec![Dest::Leave; width];

    for (y, line) in input.lines().enumerate() {
        let mut left_point = Point::new(0, y);
        let mut left = Dest::Leave;
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                if x > 0 {
                    let right = Point::new(x - 1, y);
                    for i in left_point.x..x {
                        grid[(i, y)][Direction4::West as usize] = left;
                        grid[(i, y)][Direction4::East as usize] = Dest::Turn(right);
                    }
                }

                left_point.x = x + 1;
                left = Dest::Turn(left_point);

                if y > 0 {
                    let below = Point::new(x, y - 1);
                    let above = above_points[x];
                    for i in above.y..y {
                        grid[(x, i)][Direction4::North as usize] = above_dests[x];
                        grid[(x, i)][Direction4::South as usize] = Dest::Turn(below);
                    }
                }

                above_points[x].y = y + 1;
                above_dests[x] = Dest::Turn(above_points[x]);
            } else {
                if b == b'^' {
                    start.x = x;
                    start.y = y;
                }
            }
        }

        for i in left_point.x..width {
            grid[(i, y)][Direction4::West as usize] = left;
            grid[(i, y)][Direction4::East as usize] = Dest::Leave;
        }
    }

    for i in 0..width {
        let above_dest = above_dests[i];
        for j in above_points[i].y..height {
            grid[(i, j)][Direction4::North as usize] = above_dest;
            grid[(i, j)][Direction4::South as usize] = Dest::Leave;
        }
    }

    (DirectionGrid { grid }, start)
}

fn visit_range(
    from: &Point,
    to: &Point,
    direction: &Direction4,
    visited: &mut PointGrid<bool>,
) -> usize {
    travel(&from, &to, &direction)
        .iter()
        .map(|point| {
            if !visited[*point] {
                visited[*point] = true;
                return true;
            }
            false
        })
        .filter(|b| *b)
        .count()
}

pub fn part_1(_input: &str) -> Solution {
    let (map, start) = parse(_input);

    let mut visited = PointGrid::empty(false, map.grid.height, map.grid.width);

    let mut direction = Direction4::North;
    let mut previous = start;
    let mut result: usize = 0;

    while let Dest::Turn(next) = map.grid[previous][direction as usize] {
        result += visit_range(&previous, &next, &direction, &mut visited);
        direction = direction.rotate_clockwise();
        previous = next;
    }

    let h = map.grid.height - 1;
    let w = map.grid.width - 1;

    result += match direction {
        Direction4::North => visit_range(
            &previous,
            &Point::new(previous.x, 0),
            &direction,
            &mut visited,
        ),
        Direction4::East => visit_range(
            &previous,
            &Point::new(w, previous.y),
            &direction,
            &mut visited,
        ),
        Direction4::South => visit_range(
            &previous,
            &Point::new(previous.x, h),
            &direction,
            &mut visited,
        ),
        Direction4::West => visit_range(
            &previous,
            &Point::new(0, previous.y),
            &direction,
            &mut visited,
        ),
    };

    result.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 41)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(5101)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn visit_range_test_stone(
    from: &Point,
    to: &Point,
    direction: &Direction4,
    map: &DirectionGrid,
    visited: &mut PointGrid<[bool; 4]>,
) -> usize {
    let points = travel(from, to, direction);

    let mut result = 0;

    for i in 0..points.len() - 1 {
        let current = points[i];
        let next = points[i];

        visited[current][*direction as usize] = true;

        if !visited[next].iter().any(|b| *b) {
            if is_loop(&current, &next, direction, map, visited) {
                result += 1;
            }
        }
    }

    result
}

fn is_loop(
    from: &Point,
    stone: &Point,
    direction: &Direction4,
    map: &DirectionGrid,
    visited: &mut PointGrid<[bool; 4]>,
) -> bool {
    let mut previous = *from;
    let mut direction = *direction;
    let mut branch_visited = PointGrid::empty([false; 4], visited.height, visited.width);

    loop {
        let next = map.grid[previous][direction as usize];
    }
}

fn travel(from: &Point, to: &Point, direction: &Direction4) -> Vec<Point> {
    match direction {
        Direction4::North => (to.y..=from.y)
            .rev()
            .map(|i| Point::new(from.x, i))
            .collect(),
        Direction4::East => (from.x..=to.y).map(|i| Point::new(i, from.y)).collect(),
        Direction4::South => (from.y..=to.y).map(|i| Point::new(from.x, i)).collect(),
        Direction4::West => (to.x..=from.y)
            .rev()
            .map(|i| Point::new(i, from.y))
            .collect(),
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (map, start) = parse(_input);

    let mut visited = PointGrid::empty([false; 4], map.grid.height, map.grid.width);

    let mut direction = Direction4::North;
    let mut previous = start;
    let mut result: usize = 0;

    while let Dest::Turn(next) = map.grid[previous][direction as usize] {
        result += visit_range_test_stone(&previous, &next, &direction, &map, &mut visited);
        direction = direction.rotate_clockwise();
        previous = next;
    }

    result.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_edge.txt"), 4)]
    fn edge_case_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(include_str!("_test.txt"), 6)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1951)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
