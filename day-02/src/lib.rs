use shared::parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let mut bytes = line.bytes();
            let mut previous: u8 = bytes.next_number().unwrap();
            let current: u8 = bytes.next_number().unwrap();

            let diff = previous.abs_diff(current);
            if diff < 1 || diff > 3 {
                return None;
            }
            let is_descending = previous > current;

            previous = current;

            while let Some(current) = bytes.next_number() {
                let diff = previous.abs_diff(current);
                if diff < 1
                    || diff > 3
                    || is_descending && previous < current
                    || !is_descending && previous > current
                {
                    return None;
                }

                previous = current;
            }

            Some(true)
        })
        .count()
}

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 2)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(236)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let mut bytes = line.bytes();
            let mut deltas = Vec::new();
            let mut s = 0;

            let mut previous: i16 = bytes.next_number().unwrap();
            while let Some(current) = bytes.next_number() {
                let d: i16 = current - previous;
                deltas.push(d);
                s += d.signum();
                previous = current;
            }

            if s < 0 {
                let mut iter = deltas.iter_mut();
                while let Some(current) = iter.next() {
                    *current = -*current;
                }
            }

            let outliers: Vec<usize> = deltas
                .iter()
                .enumerate()
                .filter_map(|(i, d)| {
                    if *d < 1 || *d > 3 {
                        return Some(i);
                    }
                    None
                })
                .collect();

            if outliers.len() > 2 {
                return None;
            }

            if outliers.len() == 2 {
                if outliers[1] - outliers[0] != 1 {
                    return None;
                }
                let d = deltas.get(outliers[0]).unwrap() + deltas.get(outliers[1]).unwrap();
                if d < 1 || d > 3 {
                    return None;
                }
            }

            if outliers.len() == 1 {
                let i = outliers[0];
                if i != 0 && i != deltas.len() - 1 {
                    let d_1 = deltas.get(i).unwrap() + deltas.get(i + 1).unwrap();
                    let d_2 = deltas.get(i).unwrap() + deltas.get(i - 1).unwrap();
                    if (d_1 < 1 || d_1 > 3) && (d_2 < 1 || d_2 > 3) {
                        return None;
                    }
                }
            }

            Some(true)
        })
        .count()
}

pub fn part_2(_input: &str) -> Solution {
    solve_2(_input).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 4)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(308)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
