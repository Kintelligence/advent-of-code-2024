use std::{collections::BinaryHeap, iter::from_fn, usize};

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
    let mut buckets = vec![Vec::new(); SIZE1];
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

#[derive(Eq, PartialEq)]
struct State {
    point: Point,
    distance: usize,
}

impl State {
    pub fn new(point: Point, distance: usize) -> Self {
        Self { point, distance }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

fn is_traversable(
    map: &mut Grid<bool>,
    visited: &mut Grid<bool>,
    queue: &mut BinaryHeap<State>,
    bytes: &Vec<Point>,
    count: usize,
    max_x: usize,
    max_y: usize,
) -> bool {
    map.vec.fill(false);
    visited.vec.fill(false);
    queue.clear();

    for byte in bytes.iter().take(count) {
        map[*byte] = true;
    }

    let start = Point::new(0, 0);
    let goal = Point::new(max_x, max_y);

    visited[start] = true;
    queue.push(State::new(start, usize::MAX));

    while let Some(state) = queue.pop() {
        if state.point == goal {
            return true;
        }

        for next in map.adjacent_four(state.point) {
            if !map[next] {
                if visited[next] {
                    continue;
                }

                queue.push(State::new(next, next.distance_to(goal)));
                visited[next] = true;
            }
        }
    }

    false
}

fn solve_2(input: &str, max_x: usize, max_y: usize) -> Solution {
    let bytes = parse(input);
    let mut map = Grid::filled(false, max_y + 1, max_x + 1);
    let mut visited = map.same_size_with(false);
    let mut queue = BinaryHeap::new();

    let mut min = 0;
    let mut max = bytes.len() - 1;

    loop {
        let x = (min + max) / 2;
        if is_traversable(&mut map, &mut visited, &mut queue, &bytes, x, max_x, max_y) {
            min = x + 1;
        } else {
            max = x - 1;
        }

        if max == min {
            if is_traversable(
                &mut map,
                &mut visited,
                &mut queue,
                &bytes,
                max,
                max_x,
                max_y,
            ) {
                return bytes[max].into();
            } else {
                return bytes[max - 1].into();
            }
        }
    }
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
