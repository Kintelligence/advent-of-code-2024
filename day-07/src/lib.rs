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

fn check(calibration: &Calibration) -> bool {
    check_recursive(calibration, calibration.result, calibration.inputs.len())
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
        .filter_map(|c| if check(&c) { Some(c.result) } else { None })
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

fn check_2(calibration: &Calibration) -> bool {
    check_recursive_2(calibration, calibration.inputs[0], 1)
}

fn check_recursive_2(calibration: &Calibration, sum: u64, i: usize) -> bool {
    if i == calibration.inputs.len() {
        return sum == calibration.result;
    }

    if sum > calibration.result {
        return false;
    }

    if check_recursive_2(calibration, sum + calibration.inputs[i], i + 1) {
        return true;
    }

    if check_recursive_2(calibration, sum * calibration.inputs[i], i + 1) {
        return true;
    }

    if let Some(concat_sum) = concatenate(sum, calibration.inputs[i]) {
        if check_recursive_2(calibration, concat_sum, i + 1) {
            return true;
        }
    }

    false
}

fn concatenate(mut left: u64, mut right: u64) -> Option<u64> {
    if right == 0 {
        return left.checked_mul(10);
    }

    let result = right;

    while right > 0 {
        right /= 10;
        if let Some(l) = left.checked_mul(10) {
            left = l;
        } else {
            return None;
        }
    }

    return Some(left + result);
}

pub fn part_2(_input: &str) -> Solution {
    let calibrations = parse(_input);
    calibrations
        .iter()
        .filter_map(|c| if check_2(&c) { Some(c.result) } else { None })
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

    #[test_case(123, 321, 123321)]
    fn concat_test(left: u64, right: u64, expected: u64) {
        assert_eq!(concatenate(left, right), Some(expected));
    }
}
