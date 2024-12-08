use std::iter::from_fn;

use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Debug)]
struct Calibration {
    result: u64,
    inputs: Vec<u64>,
}

fn parse(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();

            Calibration {
                result: bytes.next_number().unwrap(),
                inputs: from_fn(|| bytes.next_number()).collect(),
            }
        })
        .collect()
}

fn check_recursive(calibration: &Calibration, remainder: u64, i: usize) -> bool {
    if i == 0 {
        return remainder == 0;
    }

    if remainder % calibration.inputs[i - 1] == 0 {
        if check_recursive(calibration, remainder / calibration.inputs[i - 1], i - 1) {
            return true;
        }
    }

    if let Some(sub_remainder) = remainder.checked_sub(calibration.inputs[i - 1]) {
        if check_recursive(calibration, sub_remainder, i - 1) {
            return true;
        }
    }

    false
}

pub fn part_1(_input: &str) -> Solution {
    let calibrations = parse(_input);
    calibrations
        .iter()
        .filter_map(|c| {
            if check_recursive(&c, c.result, c.inputs.len()) {
                Some(c.result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 3749)]
    fn example_input(input: &str, expected: u64) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(4364915411363)]
    fn real_input(expected: u64) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn split(mut lhs: u64, mut rhs: u64) -> Option<u64> {
    while rhs > 0 {
        if lhs % 10 != rhs % 10 {
            return None;
        }
        lhs /= 10;
        rhs /= 10;
    }
    Some(lhs)
}

fn check_recursive_2(calibration: &Calibration, remainder: u64, i: usize) -> bool {
    if i == 0 {
        return remainder == 0;
    }

    if remainder % calibration.inputs[i - 1] == 0 {
        if check_recursive_2(calibration, remainder / calibration.inputs[i - 1], i - 1) {
            return true;
        }
    }

    if let Some(sub_remainder) = remainder.checked_sub(calibration.inputs[i - 1]) {
        if check_recursive_2(calibration, sub_remainder, i - 1) {
            return true;
        }
    }

    if let Some(split_remainder) = split(remainder, calibration.inputs[i - 1]) {
        if check_recursive_2(calibration, split_remainder, i - 1) {
            return true;
        }
    }

    false
}
pub fn part_2(_input: &str) -> Solution {
    let calibrations = parse(_input);
    calibrations
        .iter()
        .filter_map(|c| {
            if check_recursive_2(&c, c.result, c.inputs.len()) {
                Some(c.result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 11387)]
    fn example_input(input: &str, expected: u64) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(include_str!("_edge.txt"), 0)]
    fn edge_input(input: &str, expected: u64) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(38322057216320)]
    fn real_input(expected: u64) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
