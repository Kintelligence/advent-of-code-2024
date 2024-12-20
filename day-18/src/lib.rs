use std::iter::from_fn;

use grid::Grid;
use parse::Parsable;
use points::point::Point;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Vec<Point> {
    let mut bytes = input.bytes();
    from_fn(|| bytes.next_number()).collect()
}

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input, 70, 70, 1024)
}

const SIZE1: usize = 2;

fn solve_1(input: &str, max_x: usize, max_y: usize, count: usize) -> Solution {
    let bytes = parse(input);
    let mut map = Grid::filled(false, max_y + 1, max_x + 1);

    for byte in bytes.iter().take(count) {
        map[*byte] = true;
    }

    let start = Point::new(0, 0);
    let goal = Point::new(max_x, max_y);
    let mut costs: Grid<usize> = map.same_size_with(usize::MAX);
    let mut buckets = vec![Vec::with_capacity(1000); SIZE1];
    let mut bucket = 0;
    let mut remaining = 1;

    costs[start] = 0;
    buckets[bucket].push((start, 0));

    while remaining > 0 {
        while let Some((point, cost)) = buckets[bucket % SIZE1].pop() {
            remaining -= 1;

            if costs[point] < cost {
                continue;
            }

            if point == goal {
                return cost.into();
            }

            for next in map.adjacent_four(point) {
                if !map[next] {
                    if costs[next] <= cost + 1 {
                        continue;
                    }

                    buckets[(cost + 1) % SIZE1].push((next, cost + 1));
                    costs[next] = cost + 1;
                    remaining += 1;
                }
            }
        }

        bucket += 1;
    }

    Solution::None
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 22)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(solve_1(input, 6, 6, 12), expected.into());
    }

    #[test_case(276)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn fill(point: Point, visited: &mut Grid<bool>, map: &Grid<bool>) -> bool {
    visited[point] = true;
    if point.x == 0 && point.y == 0 {
        return true;
    }

    for next in map.adjacent_four(point) {
        if !map[next] && !visited[next] {
            if fill(next, visited, map) {
                return true;
            }
        }
    }

    return false;
}

fn solve_2(input: &str, max_x: usize, max_y: usize) -> Solution {
    let bytes = parse(input);
    let mut map = Grid::filled(false, max_y + 1, max_x + 1);
    let mut visited = map.same_size_with(false);
    for b in bytes.iter() {
        map[*b] = true;
    }

    fill(Point::new(max_x, max_y), &mut visited, &map);

    for i in (0..bytes.len() - 1).rev() {
        let point = bytes[i];
        map[point] = false;

        if map.adjacent_four(point).any(|n| visited[n]) && fill(point, &mut visited, &map) {
            return bytes[i].into();
        }
    }
    Solution::None
}

pub fn part_2(_input: &str) -> Solution {
    solve_2(_input, 70, 70)
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), Point::new(6,1))]
    fn example_input(input: &str, expected: Point) {
        assert_eq!(solve_2(input, 6, 6), expected.into());
    }

    #[test_case(Point::new(60, 37))]
    fn real_input(expected: Point) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
