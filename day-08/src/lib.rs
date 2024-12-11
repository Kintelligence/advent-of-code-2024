use fxhash::FxHashMap as HashMap;

use ipoint::IPoint;
use point::Point;
use point_grid::PointGrid;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Vec<Vec<IPoint>>, usize, usize) {
    let mut y = 0;

    let mut translation = HashMap::default();
    let mut next_id: usize = 0;
    let mut sets: Vec<Vec<IPoint>> = Vec::new();
    let mut x = 0;

    for line in input.lines() {
        x = 0;
        for byte in line.bytes() {
            if byte != b'.' {
                if let Some(id) = translation.get(&byte) {
                    let vec: &mut Vec<IPoint> = sets.get_mut(*id).unwrap();
                    vec.push(IPoint::new(x, y));
                } else {
                    translation.insert(byte, next_id);
                    sets.push(vec![IPoint::new(x, y)]);
                    next_id += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    (sets, x as usize, y as usize)
}

pub fn part_1(_input: &str) -> Solution {
    let (sets, width, height) = parse(_input);
    let mut map = PointGrid::from_vec(vec![false; width * height], height);
    let mut antinodes: usize = 0;

    for set in sets {
        for i in 0..(set.len() - 1) {
            let current = set[i];

            for n in (i + 1)..set.len() {
                let other = set[n];
                if let Some(offset) = current.checked_sub(other) {
                    if let Some(antinode) = other.checked_sub(offset) {
                        if let Some(point) = Point::from(antinode) {
                            if map.is_within_bounds(point) {
                                if !map[point] {
                                    map.insert(point, true);
                                    antinodes += 1;
                                }
                            }
                        }
                    }
                    if let Some(antinode) = current.checked_add(offset) {
                        if let Some(point) = Point::from(antinode) {
                            if map.is_within_bounds(point) {
                                if !map[point] {
                                    map.insert(point, true);
                                    antinodes += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    antinodes.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 14)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(293)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (sets, width, height) = parse(_input);
    let mut map = PointGrid::from_vec(vec![false; width * height], height);
    let mut antinodes: usize = 0;

    for set in sets {
        for i in 0..(set.len() - 1) {
            let current = set[i];
            if let Some(current_point) = Point::from(current) {
                if !map[current_point] {
                    map.insert(current_point, true);
                    antinodes += 1;
                }
            }

            for n in (i + 1)..set.len() {
                let other = set[n];
                if let Some(other_point) = Point::from(other) {
                    if !map[other_point] {
                        map.insert(other_point, true);
                        antinodes += 1;
                    }
                }

                if let Some(offset) = current.checked_sub(other) {
                    let mut point = current;
                    loop {
                        if let Some(antinode) = point.checked_add(offset) {
                            if let Some(antinode_point) = Point::from(antinode) {
                                if map.is_within_bounds(antinode_point) {
                                    if !map[antinode_point] {
                                        map.insert(antinode_point, true);
                                        antinodes += 1;
                                    }
                                    point = antinode;
                                    continue;
                                }
                            }
                        }
                        break;
                    }
                    point = other;
                    loop {
                        if let Some(antinode) = point.checked_sub(offset) {
                            if let Some(antinode_point) = Point::from(antinode) {
                                if map.is_within_bounds(antinode_point) {
                                    if !map[antinode_point] {
                                        map.insert(antinode_point, true);
                                        antinodes += 1;
                                    }
                                    point = antinode;
                                    continue;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    antinodes.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 34)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(934)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
