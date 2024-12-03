#![feature(iter_advance_by)]
use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn solve_1(input: &str) -> usize {
    let mut s = 0;
    for line in input.lines() {
        let mut l = line;
        while let Some(i) = l.find("mul") {
            l = &l[i + 3..];
            let mut iter = l.bytes();

            let n = iter.next();
            if n.is_none() {
                continue;
            }

            let c = n.unwrap();
            if c != b'(' {
                continue;
            }

            let (l, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
            if l.is_none() || n.is_none() {
                continue;
            }

            let a = l.unwrap();
            if a >= 1000 {
                continue;
            }

            let c = n.unwrap();
            if c != b',' {
                continue;
            }

            let (r, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
            if r.is_none() || n.is_none() {
                continue;
            }

            let b = r.unwrap();
            if b >= 1000 {
                continue;
            }

            let c = n.unwrap();
            if c != b')' {
                continue;
            }

            s += a * b;
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

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), (181345830 as usize).into());
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

            let n = iter.next();
            if n.is_none() {
                continue;
            }

            let c = n.unwrap();
            if c != b'(' {
                continue;
            }

            let (l, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
            if l.is_none() || n.is_none() {
                continue;
            }

            let a = l.unwrap();
            if a >= 1000 {
                continue;
            }

            let c = n.unwrap();
            if c != b',' {
                continue;
            }

            let (r, n): (Option<usize>, Option<u8>) = iter.next_number_strict();
            if r.is_none() || n.is_none() {
                continue;
            }

            let b = r.unwrap();
            if b >= 1000 {
                continue;
            }

            let c = n.unwrap();
            if c != b')' {
                continue;
            }

            s += a * b;
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

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), (98729041 as usize).into());
    }
}
