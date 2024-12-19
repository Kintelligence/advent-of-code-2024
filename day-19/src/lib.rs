use std::{iter::from_fn, ops::IndexMut};

use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Clone, Debug)]
struct Node {
    end: bool,
    root: bool,
    children: Vec<Option<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: vec![None; 5],
            end: false,
            root: false,
        }
    }
}

fn parse_pattern<T: Iterator<Item = u8>>(bytes: &mut T, current: &mut Node) {
    if let Some(i) = parse_colour(bytes) {
        if current.children[i as usize].is_none() {
            current.children[i as usize] = Some(Node::new());
        }

        if let Some(child) = &mut current.children.index_mut(i as usize) {
            parse_pattern(bytes, child);
        }
    } else {
        if !current.root {
            current.end = true;
        }
    }
}

fn parse_colour<T: Iterator<Item = u8>>(bytes: &mut T) -> Option<u8> {
    if let Some(b) = bytes.next() {
        if b.is_ascii_alphabetic() {
            return match b {
                b'w' => Some(0),
                b'u' => Some(1),
                b'b' => Some(2),
                b'r' => Some(3),
                b'g' => Some(4),
                _ => None,
            };
        }
    }
    None
}

fn parse(input: &str) -> (Node, Vec<Vec<u8>>) {
    let mut lines = input.lines();

    let mut root = Node::new();
    root.root = true;

    let mut bytes = lines.next().unwrap().bytes();
    loop {
        parse_pattern(&mut bytes, &mut root);
        if let Some(_) = bytes.next() {
            continue;
        }
        break;
    }

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

    (root, designs)
}

fn solve_1(
    root: &Node,
    current: Option<&Node>,
    design: &Vec<u8>,
    index: usize,
    cache: &mut Vec<bool>,
) -> bool {
    if index >= design.len() {
        return false;
    }

    if current.is_none() && cache[index] {
        return false;
    }

    if let Some(child) = &current.unwrap_or(root).children[design[index] as usize] {
        if child.end {
            if index == design.len() - 1 {
                return true;
            }
            return solve_1(root, Some(&child), design, index + 1, cache)
                || solve_1(root, None, design, index + 1, cache);
        } else {
            return solve_1(root, Some(&child), design, index + 1, cache);
        }
    }

    if current.is_none() {
        cache[index] = true;
    }

    false
}

pub fn part_1(_input: &str) -> Solution {
    let (root, designs) = parse(_input);
    designs
        .iter()
        .map(|d| solve_1(&root, None, d, 0, &mut vec![false; d.len()]))
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

fn solve_2(
    root: &Node,
    current: Option<&Node>,
    design: &Vec<u8>,
    index: usize,
    cache: &mut Vec<Option<usize>>,
) -> usize {
    if index >= design.len() {
        return 0;
    }

    if current.is_none() {
        if let Some(cache_result) = cache[index] {
            return cache_result;
        }
    }

    let mut result = 0;
    if let Some(child) = &current.unwrap_or(root).children[design[index] as usize] {
        if child.end {
            if index == design.len() - 1 {
                result = 1;
            } else {
                result = solve_2(root, Some(&child), design, index + 1, cache)
                    + solve_2(root, None, design, index + 1, cache);
            }
        } else {
            result = solve_2(root, Some(&child), design, index + 1, cache);
        }
    }

    if current.is_none() {
        cache[index] = Some(result);
    }

    result
}

pub fn part_2(_input: &str) -> Solution {
    let (root, designs) = parse(_input);
    designs
        .iter()
        .map(|d| solve_2(&root, None, d, 0, &mut vec![None; d.len()]))
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
