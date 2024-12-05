use std::collections::{HashMap, HashSet};

use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Debug)]
struct Rule {
    x: u8,
    y: u8,
}

impl Rule {
    pub fn parse(input: &str) -> Self {
        let mut bytes = input.bytes();
        Rule {
            x: bytes.next_number().unwrap(),
            y: bytes.next_number().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<u8>,
    map: HashMap<u8, u8>,
}

impl Update {
    pub fn parse(input: &str) -> Self {
        let mut bytes = input.bytes();
        let mut pages = Vec::new();
        let mut map = HashMap::new();

        let mut i = 0;
        while let Some(page) = bytes.next_number() {
            pages.push(page);
            map.insert(page, i);
            i += 1;
        }

        Update { pages, map }
    }

    pub fn validate_rule(&self, rule: &Rule) -> Option<bool> {
        if let Some(x) = self.map.get(&rule.x) {
            if let Some(y) = self.map.get(&rule.y) {
                return Some(y > x);
            }
        }

        None
    }

    pub fn validate_rules(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            if let Some(validation) = self.validate_rule(&rule) {
                if !validation {
                    return false;
                }
            }
        }

        true
    }
}

fn mid_page(vec: &Vec<u8>) -> u8 {
    let mid = (vec.len() - 1) / 2;
    vec[mid]
}

fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut line_iter = input.lines().into_iter();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        rules.push(Rule::parse(line));
    }

    while let Some(line) = line_iter.next() {
        updates.push(Update::parse(line));
    }

    (rules, updates)
}

pub fn part_1(_input: &str) -> Solution {
    let (rules, updates) = parse(_input);
    let mut sum: usize = 0;
    for update in updates {
        if update.validate_rules(&rules) {
            sum += mid_page(&update.pages) as usize;
        }
    }

    sum.into()
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

pub fn part_2(_input: &str) -> Solution {
    let (rules, updates) = parse(_input);

    let mut sum: usize = 0;

    for update in updates {
        let mut valid_rules = Vec::new();
        let mut sorted = true;

        for rule in rules.iter() {
            if let Some(x) = update.map.get(&rule.x) {
                if let Some(y) = update.map.get(&rule.y) {
                    valid_rules.push(rule);
                    if x >= y {
                        sorted = false;
                    }
                }
            }
        }

        if sorted {
            continue;
        }

        let mut x_map: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut y_map: HashMap<u8, HashSet<u8>> = HashMap::new();

        for rule in valid_rules.iter() {
            if let Some(list) = x_map.get_mut(&rule.x) {
                list.insert(rule.y);
            } else {
                let mut list = HashSet::new();
                list.insert(rule.y);
                x_map.insert(rule.x, list);
            }

            if let Some(list) = y_map.get_mut(&rule.y) {
                list.insert(rule.x);
            } else {
                let mut list = HashSet::new();
                list.insert(rule.x);
                y_map.insert(rule.y, list);
            }
        }

        let mut pages = HashSet::new();
        for page in update.pages {
            pages.insert(page);
        }

        let mut front = Vec::new();
        let mut back = Vec::new();

        while pages.len() > 0 {
            let mut to_remove = Vec::new();
            for page in pages.iter() {
                if let Some(list) = x_map.get(&page) {
                    if list.is_empty() {
                        back.push(*page);
                        to_remove.push(*page);
                        remove_page_from_maps(&mut x_map, &mut y_map, *page);
                        continue;
                    }
                } else {
                    back.push(*page);
                    to_remove.push(*page);
                    remove_page_from_maps(&mut x_map, &mut y_map, *page);
                    continue;
                }

                if let Some(list) = y_map.get(&page) {
                    if list.is_empty() {
                        front.push(*page);
                        to_remove.push(*page);
                        remove_page_from_maps(&mut x_map, &mut y_map, *page);
                    }
                } else {
                    front.push(*page);
                    to_remove.push(*page);
                    remove_page_from_maps(&mut x_map, &mut y_map, *page);
                }
            }

            for i in to_remove {
                pages.remove(&i);
            }
        }

        back.reverse();
        front.append(&mut back);

        let mid = (front.len() - 1) / 2;
        sum += front[mid] as usize;
    }

    sum.into()
}

fn remove_page_from_maps(
    x_map: &mut HashMap<u8, HashSet<u8>>,
    y_map: &mut HashMap<u8, HashSet<u8>>,
    id: u8,
) {
    if let Some(list) = x_map.remove(&id) {
        for node in list {
            if let Some(inner_list) = y_map.get_mut(&node) {
                inner_list.remove(&id);
            }
        }
    }

    if let Some(list) = y_map.remove(&id) {
        for node in list {
            if let Some(inner_list) = x_map.get_mut(&node) {
                inner_list.remove(&id);
            }
        }
    }
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
