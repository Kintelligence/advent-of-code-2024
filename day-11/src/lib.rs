use std::{collections::HashMap, iter::from_fn};

use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Vec<usize> {
    let mut bytes = input.bytes();
    from_fn(|| bytes.next_number()).collect()
}

fn split(stone: usize) -> Option<(usize, usize)> {
    let mut digits = 0;
    let mut number = stone;

    while number > 0 {
        number /= 10;
        digits += 1;
    }

    if digits % 2 != 0 {
        return None;
    }

    let d = 10usize.pow(digits / 2);
    return Some((stone / d, stone % d));
}

fn next_1(stone: usize, iterations: usize, cache: &mut HashMap<usize, [usize; 26]>) -> usize {
    if stone == 0 {
        return count_1(1, iterations - 1, cache);
    } else if let Some((left, right)) = split(stone) {
        return count_1(left, iterations - 1, cache) + count_1(right, iterations - 1, cache);
    }
    return count_1(stone * 2024, iterations - 1, cache);
}

fn count_1(stone: usize, iterations: usize, cache: &mut HashMap<usize, [usize; 26]>) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(map) = cache.get_mut(&stone) {
        let result = map[iterations];
        if result != 0 {
            return result;
        }
    } else {
        cache.insert(stone, [0; 26]);
    }

    let result = next_1(stone, iterations, cache);
    cache.get_mut(&stone).unwrap()[iterations] = result;
    result
}

pub fn part_1(_input: &str) -> Solution {
    let mut cache = HashMap::new();
    parse(_input)
        .iter()
        .map(|rock| count_1(*rock, 25, &mut cache))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 55312)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(194557)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn next_2(stone: usize, iterations: usize, cache: &mut HashMap<usize, Vec<usize>>) -> usize {
    if stone == 0 {
        return count_2(1, iterations - 1, cache);
    } else if let Some((left, right)) = split(stone) {
        return count_2(left, iterations - 1, cache) + count_2(right, iterations - 1, cache);
    }
    return count_2(stone * 2024, iterations - 1, cache);
}

fn count_2(stone: usize, iterations: usize, cache: &mut HashMap<usize, Vec<usize>>) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(map) = cache.get_mut(&stone) {
        let result = map[iterations];
        if result != 0 {
            return result;
        }
    } else {
        cache.insert(stone, vec![0; 76]);
    }

    let result = next_2(stone, iterations, cache);
    cache.get_mut(&stone).unwrap()[iterations] = result;
    result
}

pub fn part_2(_input: &str) -> Solution {
    let mut cache = HashMap::new();
    parse(_input)
        .iter()
        .map(|rock| count_2(*rock, 75, &mut cache))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 65601038650482)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(231532558973909)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
