use std::iter::from_fn;

use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = vec![Vec::new(); 100];
    let mut updates = Vec::new();

    let mut line_iter = input.lines().into_iter();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut bytes = line.bytes();
        let x: usize = bytes.next_number().unwrap();
        let y: usize = bytes.next_number().unwrap();
        rules[x].push(y);
    }

    while let Some(line) = line_iter.next() {
        let mut bytes = line.bytes();
        updates.push(from_fn(|| bytes.next_number()).collect());
    }

    (rules, updates)
}

fn check_1(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() - 1 {
        if !rules[update[i]].contains(&update[i + 1]) {
            return None;
        }
    }
    let mid = (update.len() - 1) / 2;
    Some(update[mid])
}

pub fn part_1(_input: &str) -> Solution {
    let (rules, updates) = parse(_input);
    updates
        .iter()
        .filter_map(|update| check_1(update, &rules))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 143)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(5452)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn find_next_page(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() {
        let rule = &rules[update[i]];
        let mut found = true;
        for n in 0..update.len() {
            if n == i {
                continue;
            }

            if rule.contains(&update[n]) {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }
    None
}

fn check_2(update: &mut Vec<usize>, rules: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for _ in 0..=update.len() / 2 {
        if let Some(page_index) = find_next_page(update, rules) {
            result = update.remove(page_index);
        }
    }
    result
}

pub fn part_2(_input: &str) -> Solution {
    let (rules, mut updates) = parse(_input);
    updates
        .iter_mut()
        .filter(|update| check_1(update, &rules).is_none())
        .map(|mut update| check_2(&mut update, &rules))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 123)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(4598)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
