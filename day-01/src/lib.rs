use std::collections::HashMap;

use shared::parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Vec<u16>, Vec<u16>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut bytes = line.bytes();
        left.push(bytes.next_number().unwrap());
        right.push(bytes.next_number().unwrap());
    }

    (left, right)
}

fn solve_1(left: &mut Vec<u16>, right: &mut Vec<u16>) -> usize {
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r) as usize)
        .sum()
}

pub fn part_1(_input: &str) -> Solution {
    let (mut left, mut right) = parse(_input);
    solve_1(&mut left, &mut right).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 11)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(2756096)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(left: &mut Vec<u16>, right: &mut Vec<u16>) -> usize {
    let mut hash = HashMap::new();
    for n in left {
        if let Some(i) = hash.get(&n) {
            hash.insert(n, i + 1);
        } else {
            hash.insert(n, 1);
        }
    }

    let mut sum: usize = 0;
    for n in right {
        if let Some(i) = hash.get(&n) {
            sum += i * *n as usize;
        }
    }

    sum
}

pub fn part_2(_input: &str) -> Solution {
    let (mut left, mut right) = parse(_input);
    solve_2(&mut left, &mut right).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 31)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(23117829)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
