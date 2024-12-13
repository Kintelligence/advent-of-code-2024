#![feature(iter_advance_by)]
use parse::ParsableStrict;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn multiply<T: Iterator<Item = u8>>(iter: &mut T) -> Option<usize> {
    let n = iter.next();
    if n.is_none() {
        return None;
    }

    let c = n.unwrap();
    if c != b'(' {
        return None;
    }

    let (l, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
    if l.is_none() || n.is_none() {
        return None;
    }

    let a = l.unwrap();
    if a >= 1000 {
        return None;
    }

    let c = n.unwrap();
    if c != b',' {
        return None;
    }

    let (r, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
    if r.is_none() || n.is_none() {
        return None;
    }

    let b = r.unwrap();
    if b >= 1000 {
        return None;
    }

    let c = n.unwrap();
    if c != b')' {
        return None;
    }

    return Some(a * b);
}

fn solve_1(input: &str) -> usize {
    let mut s = 0;
    for line in input.lines() {
        let mut l = line;
        while let Some(i) = l.find("mul") {
            l = &l[i + 3..];
            let mut iter = l.bytes();

            if let Some(r) = multiply(&mut iter) {
                s += r;
            }
        }
    }
    s
}

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 161)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(181345830)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(input: &str) -> usize {
    let mut s = 0;
    let mut enabled = true;
    for line in input.lines() {
        let mut l = line;
        while let Some(i) = l.find("mul") {
            let left_string = &l[0..i];
            l = &l[i + 3..];
            if let Some(d) = left_string.rfind("do") {
                if left_string.len() >= d + 5 {
                    if left_string[d + 2..d + 5] == *"n't" {
                        enabled = false;
                        continue;
                    }
                }
                enabled = true;
            }

            if !enabled {
                continue;
            }

            let mut iter = l.bytes();

            if let Some(r) = multiply(&mut iter) {
                s += r;
            }
        }
    }
    s
}

pub fn part_2(_input: &str) -> Solution {
    solve_2(_input).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 48)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(98729041)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
