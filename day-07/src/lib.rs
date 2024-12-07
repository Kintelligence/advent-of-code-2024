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

    #[test_case(0)]
    fn real_input(expected: u64) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    Solution::None
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 47)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
