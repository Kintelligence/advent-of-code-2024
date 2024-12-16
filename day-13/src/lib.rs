use parse::Parsable;
use points::ipoint::IPoint;
use shared::*;
use std::iter::from_fn;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Vec<(IPoint, IPoint, IPoint)> {
    let mut bytes = input.bytes();

    from_fn(|| {
        bytes
            .next_number()
            .zip(bytes.next_number())
            .zip(bytes.next_number())
    })
    .map(|((a, b), c)| (a, b, c))
    .collect()
}

fn check_single(vec: &IPoint, c: &IPoint) -> Option<isize> {
    let result = c.x / vec.x;
    if c.x % vec.x == 0 && c.y % vec.y == 0 && result == c.y / vec.y {
        return Some(result);
    }
    None
}

fn solve(a: &IPoint, b: &IPoint, c: &IPoint) -> Option<(isize, isize)> {
    let d_m = a.x * b.y - a.y * b.x;
    let d_mx = c.x * b.y - c.y * b.x;
    let d_my = a.x * c.y - a.y * c.x;

    if d_m == 0 {
        let x_option = check_single(a, c);
        let y_option = check_single(b, c);
        return match (x_option, y_option) {
            (None, None) => None,
            (None, Some(y)) => Some((0, y)),
            (Some(x), None) => Some((x, 0)),
            (Some(x), Some(y)) => Some(if x * 3 < y { (x, 0) } else { (0, y) }),
        };
    } else {
        if d_mx % d_m == 0 && d_my % d_m == 0 {
            let x = d_mx / d_m;
            let y = d_my / d_m;

            return Some((x, y));
        }
    }

    None
}

pub fn part_1(_input: &str) -> Solution {
    parse(_input)
        .iter()
        .filter_map(|(a, b, c)| solve(a, b, c))
        .filter_map(|(x, y)| {
            if x > 100 || y > 100 {
                None
            } else {
                Some(x * 3 + y)
            }
        })
        .sum::<isize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 480)]
    fn example_input(input: &str, expected: isize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(28262)]
    fn real_input(expected: isize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let offset = IPoint::new(10000000000000, 10000000000000);
    parse(_input)
        .iter()
        .filter_map(|(a, b, c)| solve(a, b, &(c + &offset)))
        .map(|(x, y)| x * 3 + y)
        .sum::<isize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 875318608908)]
    fn example_input(input: &str, expected: isize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(101406661266314)]
    fn real_input(expected: isize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
