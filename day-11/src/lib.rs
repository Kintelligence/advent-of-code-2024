use std::iter::from_fn;
use switching_state::SwitchingState;
use translator::Translator;

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

struct State {
    stones: Vec<usize>,
    counts: [usize; 5000],
}

impl State {
    pub fn new() -> Self {
        Self {
            stones: Vec::with_capacity(5000),
            counts: [0; 5000],
        }
    }

    pub fn clear(&mut self) {
        self.stones.clear();
        self.counts.fill(0);
    }

    pub fn add_to_next(&mut self, stone: usize, id: usize, count: usize) {
        if self.counts[id] == 0 {
            self.stones.push(stone);
        }
        self.counts[id] += count;
    }
}

fn solve(initial_stones: Vec<usize>, iterations: usize) -> usize {
    let mut translator = Translator::new();
    let mut cache = [(usize::MAX, 0, usize::MAX, 0); 5000];
    let mut state = SwitchingState::new(State::new(), State::new());
    let (_, next) = state.states();

    for stone in initial_stones {
        next.add_to_next(stone, translator.translate(stone), 1);
    }

    cache[translator.translate(0)] = (1, translator.translate(1), usize::MAX, 0);

    for _ in 0..iterations {
        state.switch();
        state.next().clear();
        let (current, next) = state.states();
        for &stone in current.stones.iter() {
            let current_id = translator.translate(stone);
            let count = current.counts[current_id];
            let (cache_left_stone, cache_left_id, cache_right_stone, cache_right_id) =
                cache[current_id];
            if cache_left_stone != usize::MAX {
                next.add_to_next(cache_left_stone, cache_left_id, count);
                if cache_right_stone != usize::MAX {
                    next.add_to_next(cache_right_stone, cache_right_id, count);
                }
            } else if let Some((left, right)) = split(stone) {
                let left_id = translator.translate(left);
                let right_id = translator.translate(right);
                cache[current_id] = (left, left_id, right, right_id);
                next.add_to_next(left, left_id, count);
                next.add_to_next(right, right_id, count);
            } else {
                let stone = stone * 2024;
                let id = translator.translate(stone);
                cache[current_id] = (stone, id, usize::MAX, 0);
                next.add_to_next(stone, id, count);
            }
        }
    }

    let (_, current) = state.states();

    current
        .stones
        .iter()
        .map(|&stone| current.counts[translator.translate(stone)])
        .sum()
}

pub fn part_1(_input: &str) -> Solution {
    solve(parse(_input), 25).into()
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

pub fn part_2(_input: &str) -> Solution {
    solve(parse(_input), 75).into()
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
