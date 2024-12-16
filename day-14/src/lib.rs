use math::solve_linear_diophantine;
use parse::Parsable;
use points::{
    ipoint::IPoint,
    traits::{Absolute, ModuloPositive},
};
use shared::*;
use std::iter::from_fn;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

struct Robot {
    p: IPoint,
    v: IPoint,
}

fn parse(input: &str) -> Vec<Robot> {
    let mut bytes = input.bytes();
    from_fn(|| bytes.next_number().zip(bytes.next_number()))
        .map(|(p, v)| Robot { p, v })
        .collect()
}

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input, 101, 103).into()
}

fn solve_1(input: &str, width: isize, height: isize) -> usize {
    let mut robots = parse(input);
    let dimensions = IPoint::new(width, height);

    let mid_width = width / 2;
    let mid_height = height / 2;

    let mut quadrants = [0; 4];

    for robot in robots.iter_mut() {
        robot.p = (robot.p + robot.v * 100).modulo_positive(dimensions);

        if robot.p.x == mid_width || robot.p.y == mid_height {
            continue;
        }

        quadrants[if robot.p.x > mid_width { 0b10 } else { 0 }
            | if robot.p.y > mid_height { 1 } else { 0 }] += 1;
    }

    quadrants.iter().product()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 12)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(solve_1(input, 11, 7), expected.into());
    }

    #[test_case(218619120)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    solve_2(_input, 101, 103).into()
}

const THRESHOLD: isize = 8000;

fn solve_2(input: &str, width: isize, height: isize) -> isize {
    let mut robots = parse(input);
    let dimensions = IPoint::new(width, height);

    let mut x_cycle_start: Option<isize> = None;
    let mut y_cycle_start: Option<isize> = None;
    let mut i = 0;

    while x_cycle_start.is_none() || y_cycle_start.is_none() {
        i += 1;
        for robot in robots.iter_mut() {
            robot.p = (robot.p + robot.v).modulo_positive(dimensions);
        }

        let variance = calculate_variance(&robots);

        if variance.x < THRESHOLD {
            x_cycle_start = Some(i);
        }

        if variance.y < THRESHOLD {
            y_cycle_start = Some(i);
        }
    }

    if let Some((l1, l2)) = x_cycle_start.zip(y_cycle_start) {
        if let Some((x, _)) = solve_linear_diophantine(width, -height, l2 - l1) {
            return x * width + l1;
        }
    }

    0
}

fn calculate_variance(robots: &Vec<Robot>) -> IPoint {
    let average = robots.iter().map(|r| r.p).sum::<IPoint>() / robots.len() as isize;
    robots.iter().map(|r| (average - r.p).absolute()).sum()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(7055)]
    fn real_input(expected: isize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
