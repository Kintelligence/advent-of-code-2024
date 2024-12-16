use grid::Grid;
use points::{directions::Direction, point::Point};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Grid<u8> {
    let mut vec = Vec::new();
    let mut height = 0;

    for line in input.lines() {
        for byte in line.bytes() {
            if !byte.is_ascii_digit() {
                panic!("unexpected input");
            }
            vec.push(byte - b'0');
        }
        height += 1;
    }

    Grid::from(vec, height)
}

fn traverse_1<I>(map: &Grid<u8>, iter: I, height: u8, seen: &mut Grid<bool>) -> usize
where
    I: Iterator<Item = (Point, Direction)>,
{
    let mut result = 0;
    let valid_points = iter.filter(|(p, _)| map[*p] == height);
    if height >= 9 {
        for (p, _) in valid_points {
            if !seen[p] {
                seen[p] = true;
                result += 1;
            }
        }
    } else {
        for (point, from) in valid_points {
            result += traverse_1(
                map,
                map.adjacent_three_in_direction(point, from.reverse()),
                height + 1,
                seen,
            );
        }
    }
    result
}

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);
    map.points()
        .filter_map(|position| {
            if map[position] == 0 {
                let mut seen = Grid::from(vec![false; map.height * map.width], map.height);
                return Some(traverse_1(
                    &map,
                    map.adjacent_four_directional(position),
                    1,
                    &mut seen,
                ));
            }
            None
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 1)]
    #[test_case(include_str!("_test_2.txt"), 2)]
    #[test_case(include_str!("_test_3.txt"), 3)]
    #[test_case(include_str!("_test_4.txt"), 4)]
    #[test_case(include_str!("_test_36.txt"), 36)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(617)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn traverse_2<I>(map: &Grid<u8>, iter: I, height: u8) -> usize
where
    I: Iterator<Item = (Point, Direction)>,
{
    let valid_points = iter.filter(|(p, _)| map[*p] == height);
    if height >= 9 {
        return valid_points.count();
    } else {
        valid_points
            .map(|(point, from)| {
                traverse_2(
                    map,
                    map.adjacent_three_in_direction(point, from.reverse()),
                    height + 1,
                )
            })
            .sum()
    }
}

pub fn part_2(_input: &str) -> Solution {
    let map = parse(_input);
    map.points()
        .filter_map(|position| {
            if map[position] == 0 {
                return Some(traverse_2(&map, map.adjacent_four_directional(position), 1));
            }
            None
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_36.txt"), 81)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1477)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
