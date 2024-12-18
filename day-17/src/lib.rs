use std::{iter::from_fn, result};

use parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut bytes = input.bytes();
    let a = bytes.next_number().unwrap();
    let b = bytes.next_number().unwrap();
    let c = bytes.next_number().unwrap();
    let program: Vec<usize> = from_fn(|| bytes.next_number()).collect();

    (a, b, c, program)
}

fn run(a: &mut usize, b: &mut usize, c: &mut usize, program: &Vec<usize>) -> String {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();
    let end = program.len();

    loop {
        if instruction_pointer + 1 >= end {
            break;
        }

        let instruction = program[instruction_pointer];
        let literal = program[instruction_pointer + 1];

        match instruction {
            0 => *a = divide(&a, combo_operand(literal, *a, *b, *c)),
            1 => *b = xor(&b, literal),
            2 => *b = modulo(combo_operand(literal, *a, *b, *c)),
            3 => {
                if *a != 0 {
                    instruction_pointer = literal;
                    continue;
                }
            }
            4 => *b = xor(&b, *c),
            5 => output.push(modulo(combo_operand(literal, *a, *b, *c))),
            6 => *b = divide(&a, combo_operand(literal, *a, *b, *c)),
            7 => *c = divide(&a, combo_operand(literal, *a, *b, *c)),
            i => {
                panic!("unexpected instruction {}", i)
            }
        }

        instruction_pointer += 2;
    }

    output
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn combo_operand(operand: usize, a: usize, b: usize, c: usize) -> usize {
    match operand {
        0..4 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => {
            panic!("Unexpected combo operand")
        }
    }
}

fn divide(register: &usize, value: usize) -> usize {
    *register >> value
}

fn xor(register: &usize, value: usize) -> usize {
    *register ^ value
}

fn modulo(value: usize) -> usize {
    value & 0b111
}

pub fn part_1(_input: &str) -> Solution {
    let (mut a, mut b, mut c, program) = parse(_input);
    run(&mut a, &mut b, &mut c, &program).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), None, None, None, "4,6,3,5,6,3,5,2,1,0")]
    #[test_case(include_str!("_example_1.txt"), None, Some(1), None, "")]
    #[test_case(include_str!("_example_2.txt"), None, None, None, "0,1,2")]
    #[test_case(include_str!("_example_3.txt"), Some(0), None, None, "4,2,5,6,7,7,7,7,3,1,0")]
    #[test_case(include_str!("_example_4.txt"), None, Some(26), None, "")]
    #[test_case(include_str!("_example_5.txt"), None, Some(44354), None, "")]
    fn example_input(
        input: &str,
        expected_a: Option<usize>,
        expected_b: Option<usize>,
        expected_c: Option<usize>,
        expected_output: &str,
    ) {
        let (mut a, mut b, mut c, program) = parse(input);
        let output = run(&mut a, &mut b, &mut c, &program);

        assert_eq!(output, expected_output.to_string());
        if let Some(expected_a) = expected_a {
            assert_eq!(a, expected_a);
        }
        if let Some(expected_b) = expected_b {
            assert_eq!(b, expected_b);
        }
        if let Some(expected_c) = expected_c {
            assert_eq!(c, expected_c);
        }
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (_, _, _, program) = parse(_input);
    if let Some(result) = solve(0, program[3], program[7], &program, program.len() - 1, 0) {
        return result.into();
    }
    Solution::None
}

fn solve(
    a: usize,
    x: usize,
    y: usize,
    output: &Vec<usize>,
    index: usize,
    end: usize,
) -> Option<usize> {
    let mut results = Vec::new();

    for b in 0..8 {
        let a = a << 3 | b;
        if b ^ x ^ y ^ (a >> (b ^ x)) & 0b111 == output[index] {
            if index == end {
                results.push(a);
            } else if let Some(result) = solve(a, x, y, output, index - 1, end) {
                results.push(result);
            }
        }
    }

    results.iter().min().copied()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_2.txt"), 117440)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
