use std::collections::HashSet;

use point::Point;
use point_vec2d::{Direction, PointVec2d};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
}

fn parse(input: &str) -> (PointVec2d<Tile>, Point) {
    let mut vec = Vec::new();

    let mut height = 0;
    let mut start = Point::new(0, 0);

    for line in input.lines() {
        for (x, byte) in line.bytes().enumerate() {
            match byte {
                b'.' => vec.push(Tile::Empty),
                b'#' => vec.push(Tile::Obstacle),
                b'^' => {
                    vec.push(Tile::Empty);
                    start.x = x;
                    start.y = height;
                }
                _ => panic!("Unexpected input"),
            }
        }

        height += 1;
    }

    (PointVec2d::from_vec(vec, height), start)
}

fn solve_1(map: PointVec2d<Tile>, start: Point) -> usize {
    let mut visited = PointVec2d::from_vec(vec![false; map.width * map.height], map.height);
    let mut current = start;
    let mut visited_count = 1;
    let mut direction = Direction::North;
    visited[current] = true;

    while let Some(next) = map.go(current, direction) {
        if map[next] == Tile::Obstacle {
            direction = direction.rotate_clockwise();
            continue;
        } else if !visited[next] {
            visited[next] = true;
            visited_count += 1;
        }

        current = next;
    }

    visited_count
}

pub fn part_1(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    solve_1(map, start).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 41)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(include_str!("_mqu.txt"), 4939)]
    fn mqu_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(5101)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(map: PointVec2d<Tile>, start: Point) -> usize {
    let mut visited = PointVec2d::from_vec(vec![[false; 4]; map.width * map.height], map.height);
    let mut current = start;
    let mut loops = HashSet::new();
    let mut direction = Direction::North;
    visited[current][0] = true;

    while let Some(next) = map.go(current, direction) {
        if map[next] == Tile::Obstacle {
            direction = direction.rotate_clockwise();
            continue;
        }

        if visited[next].iter().all(|seen| !seen) {
            let mut visited_b =
                PointVec2d::from_vec(vec![[false; 4]; map.width * map.height], map.height);
            let mut direction_b = direction.rotate_clockwise();
            let mut current_b = current;

            while let Some(next_b) = map.go(current_b, direction_b) {
                if map[next_b] == Tile::Obstacle || next_b == next {
                    direction_b = direction_b.rotate_clockwise();
                    continue;
                } else if visited[next_b][direction_b as usize]
                    || visited_b[next_b][direction_b as usize]
                {
                    loops.insert(next);
                    break;
                }

                visited_b[next_b][direction_b as usize] = true;
                current_b = next_b;
            }
        }

        visited[next][direction as usize] = true;
        current = next;
    }

    loops.len()
}

pub fn part_2(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    solve_2(map, start).into()
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

    #[test_case(include_str!("_mqu.txt"), 1434)]
    fn mqu_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1951)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
