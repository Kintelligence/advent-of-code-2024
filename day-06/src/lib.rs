use std::collections::HashSet;

use point_vec2d::{Direction, Point, PointVec2d};
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

    #[test_case(5101)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn extend_backwards(
    map: &PointVec2d<Tile>,
    visited: &mut PointVec2d<Vec<[bool; 2]>>,
    point: Point,
    direction: Direction,
) {
    let mut queue = Vec::new();
    queue.push((point, direction));

    while let Some((next_point, next_direction)) = queue.pop() {
        extend_backwards_queued(map, visited, next_point, next_direction, &mut queue);
    }
}

fn extend_backwards_queued(
    map: &PointVec2d<Tile>,
    visited: &mut PointVec2d<Vec<[bool; 2]>>,
    point: Point,
    direction: Direction,
    queue: &mut Vec<(Point, Direction)>,
) {
    let mut current = point;
    visited[current][direction as usize][1] = true;
    let reverse_direction = direction.reverse();

    while let Some(next) = map.go(current, reverse_direction) {
        let hidden_stone_direction = reverse_direction.rotate_clockwise();
        let check = visited[current][hidden_stone_direction as usize];
        if !check[0] && !check[1] {
            if let Some(hidden_stone_location) = map.go(current, hidden_stone_direction) {
                if map[hidden_stone_location] == Tile::Obstacle {
                    queue.push((current, hidden_stone_direction));
                }
            }
        }

        if map[next] == Tile::Obstacle {
            break;
        } else {
            let check = visited[next][reverse_direction.reverse() as usize];
            if check[0] || check[1] {
                break;
            } else {
                visited[next][reverse_direction.reverse() as usize][1] = true;
            }
        }

        current = next;
    }
}

fn solve_2(map: PointVec2d<Tile>, start: Point) -> usize {
    let mut visited = PointVec2d::from_vec(
        vec![vec![[false, false]; 4]; map.width * map.height],
        map.height,
    );
    let mut current = start;
    let mut loop_points = HashSet::new();
    let mut direction = Direction::North;
    extend_backwards(&map, &mut visited, current, direction);

    while let Some(next) = map.go(current, direction) {
        let hidden_stone_direction = direction.rotate_counterclockwise();
        let check = visited[current][hidden_stone_direction as usize];
        if !check[0] && !check[1] {
            if let Some(hidden_stone_location) = map.go(current, hidden_stone_direction) {
                if map[hidden_stone_location] == Tile::Obstacle {
                    extend_backwards(&map, &mut visited, current, hidden_stone_direction)
                }
            }
        }

        if map[next] == Tile::Obstacle {
            direction = direction.rotate_clockwise();
            extend_backwards(&map, &mut visited, current, direction);
            continue;
        } else {
            let mut turn = direction.rotate_clockwise();
            let check = visited[current][turn as usize];
            if check[0] || check[1] {
                if !visited[next][0][0]
                    && !visited[next][1][0]
                    && !visited[next][2][0]
                    && !visited[next][3][0]
                {
                    if !visited[next][0][1]
                        && !visited[next][1][1]
                        && !visited[next][2][1]
                        && !visited[next][3][1]
                    {
                        loop_points.insert(next);
                        dbg!(next);
                    } else {
                        let mut branch_map = PointVec2d::from_vec(
                            vec![vec![false; 4]; map.width * map.height],
                            map.height,
                        );
                        let mut branch_current = current;
                        while let Some(branch_next) = map.go(branch_current, turn) {
                            if map[branch_next] == Tile::Obstacle {
                                turn = turn.rotate_clockwise();
                                continue;
                            }

                            if branch_map[branch_next][turn as usize]
                                || visited[branch_next][turn as usize][0]
                            {
                                loop_points.insert(next);
                                dbg!(next);
                                break;
                            }

                            branch_map[branch_next][turn as usize] = true;

                            branch_current = branch_next;
                        }
                    }
                }
            }

            visited[next][direction as usize][0] = true;
        }

        current = next;
    }

    loop_points.len()
}

pub fn part_2(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    solve_2(map, start).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 4)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1776)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
