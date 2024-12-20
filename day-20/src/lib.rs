use grid::Grid;
use points::{directions::Direction, ipoint::IPoint, point::Point};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Grid<bool>, Point, Point) {
    let mut vec = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let mut y = 0;

    for line in input.lines() {
        for (x, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => vec.push(false),
                b'.' => vec.push(true),
                b'S' => {
                    vec.push(true);
                    start = Point::new(x, y);
                }
                b'E' => {
                    vec.push(true);
                    end = Point::new(x, y);
                }
                _ => panic!("Unexpected input"),
            }
        }
        y += 1;
    }

    (Grid::from(vec, y), start, end)
}

fn map_costs(
    map: &Grid<bool>,
    mut point: Point,
    end: &Point,
    mut cost: usize,
    costs: &mut Grid<Option<usize>>,
) {
    let mut direction = Direction::North;
    for (n, d) in map.adjacent_four_directional(point) {
        if map[n] {
            direction = d;
        }
    }

    loop {
        costs[point] = Some(cost);

        if point == *end {
            return;
        }

        cost += 1;
        for (next, next_direction) in map.adjacent_three_in_direction(point, direction) {
            if map[next] {
                point = next;
                direction = next_direction;
            }
        }
    }
}

fn cheat(
    costs: &mut Grid<Option<usize>>,
    mut point: Point,
    end: &Point,
    limit: usize,
    range: isize,
) -> usize {
    let mut result = 0;
    let mut direction = Direction::North;
    let mut ipoint: IPoint = point.into();
    for (n, d) in costs.adjacent_four_directional(point) {
        if let Some(_) = costs[n] {
            direction = d;
        }
    }

    if let Some(mut cost) = costs[point] {
        loop {
            costs[point] = None;
            if point == *end {
                return result;
            }
            for offset in 2..=range {
                for dest in ipoint.offset_points(offset) {
                    if let Some(dest_cost) = costs.checked_index(dest).and_then(|&o| o) {
                        if dest_cost > cost && dest_cost - cost >= limit + ipoint.distance_to(dest)
                        {
                            result += 1;
                        }
                    }
                }
            }
            for (n, d) in costs.adjacent_three_in_direction(point, direction) {
                if let Some(n_cost) = costs[n] {
                    direction = d;
                    cost = n_cost;
                    point = n;
                    ipoint = point.into();
                }
            }
        }
    }

    result
}

fn solve(input: &str, limit: usize, range: isize) -> Solution {
    let (map, start, end) = parse(input);
    let mut costs = map.same_size_with(None);

    map_costs(&map, start, &end, 0, &mut costs);
    cheat(&mut costs, start, &end, limit, range).into()
}

pub fn part_1(_input: &str) -> Solution {
    solve(_input, 100, 2)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 2)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(solve(input, 40, 2), expected.into())
    }

    #[test_case(1511)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    solve(_input, 100, 20)
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 29)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(solve(input, 72, 20), expected.into())
    }

    #[test_case(1020507)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
