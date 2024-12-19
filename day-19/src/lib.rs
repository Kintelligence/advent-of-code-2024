use std::iter::from_fn;

use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<u8>>) {
    let mut lines = input.lines();

    let mut bytes = lines.next().unwrap().bytes();

    let patterns = from_fn(|| {
        let mut t: usize = 0;
        while let Some(c) = parse_colour(&mut bytes) {
            t = (t << 3) | c as usize;
        }
        if t == 0 {
            None
        } else {
            bytes.next();
            Some(t)
        }
    })
    .collect();

    lines.next();

    let designs = from_fn(|| {
        lines.next().and_then(|l| {
            let mut bytes = l.bytes();
            let mut v = Vec::new();
            while let Some(c) = parse_colour(&mut bytes) {
                v.push(c);
            }
            Some(v)
        })
    })
    .collect();

    (patterns, designs)
}

fn parse_colour<T: Iterator<Item = u8>>(bytes: &mut T) -> Option<u8> {
    if let Some(b) = bytes.next() {
        if b.is_ascii_alphabetic() {
            return Some(match b {
                b'w' => 1,
                b'u' => 2,
                b'b' => 3,
                b'r' => 4,
                b'g' => 5,
                _ => panic!("Unexpected towel color"),
            });
        }
    }
    None
}

fn extract(design: &Vec<u8>, offset: usize, length: usize) -> Option<usize> {
    if offset + length > design.len() {
        return None;
    }
    let mut r = 0;
    for i in 0..length {
        r = r << 3 | design[offset + i] as usize;
    }
    Some(r)
}

fn try_solve(
    design: &Vec<u8>,
    patterns: &Vec<Vec<usize>>,
    offset: usize,
    cache: &mut Vec<bool>,
) -> bool {
    if offset == design.len() {
        return true;
    }

    if cache[offset] == true {
        return false;
    }

    for i in 0..patterns.len() {
        if let Some(segment) = extract(design, offset, i + 1) {
            for &pattern in patterns[i].iter() {
                if segment == pattern {
                    if try_solve(design, patterns, offset + i + 1, cache) {
                        return true;
                    }
                }
            }
        } else {
            break;
        }
    }

    cache[offset] = true;

    false
}

pub fn part_1(_input: &str) -> Solution {
    let (patterns, designs) = parse(_input);
    let mut grouped_patterns = vec![Vec::new(); 10];

    for pattern in patterns {
        let i = usize::BITS - pattern.leading_zeros();
        let n = (i as f64 / 3f64).ceil() as usize;
        grouped_patterns[n - 1].push(pattern);
    }

    for i in (0..10).rev() {
        if grouped_patterns[i].is_empty() {
            grouped_patterns.remove(i);
        } else {
            break;
        }
    }

    designs
        .iter()
        .map(|d| try_solve(&d, &grouped_patterns, 0, &mut vec![false; d.len()]))
        .filter(|&b| b)
        .count()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 6)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(358)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn try_solve_2(
    design: &Vec<u8>,
    patterns: &Vec<Vec<usize>>,
    offset: usize,
    cache: &mut Vec<Option<usize>>,
) -> usize {
    if offset == design.len() {
        return 1;
    }

    if let Some(r) = cache[offset] {
        return r;
    }

    let mut count = 0;
    for i in 0..patterns.len() {
        if let Some(segment) = extract(design, offset, i + 1) {
            for &pattern in patterns[i].iter() {
                if segment == pattern {
                    count += try_solve_2(design, patterns, offset + i + 1, cache);
                }
            }
        } else {
            break;
        }
    }

    cache[offset] = Some(count);
    count
}

pub fn part_2(_input: &str) -> Solution {
    let (patterns, designs) = parse(_input);
    let mut grouped_patterns = vec![Vec::new(); 10];

    for pattern in patterns {
        let i = usize::BITS - pattern.leading_zeros();
        let n = (i as f64 / 3f64).ceil() as usize;
        grouped_patterns[n - 1].push(pattern);
    }

    for i in (0..10).rev() {
        if grouped_patterns[i].is_empty() {
            grouped_patterns.remove(i);
        } else {
            break;
        }
    }

    designs
        .iter()
        .map(|d| try_solve_2(&d, &grouped_patterns, 0, &mut vec![None; d.len()]))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 16)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(600639829400603)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
