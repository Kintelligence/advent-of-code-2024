use std::iter::from_fn;

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
                println!("No JUMP")
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
    if let Some(a) = reverse(
        program.len() - 4,
        program.len() - 1,
        &program,
        0,
        0,
        None,
        &mut 0,
        &mut (program.len() - 4),
        &mut (program.len() - 1),
    ) {
        return a.into();
    }
    Solution::None
}

fn reverse(
    p_index: usize,
    o_index: usize,
    program: &Vec<usize>,
    a: usize,
    b: usize,
    c_option: Option<usize>,
    count: &mut usize,
    min_o: &mut usize,
    min_p: &mut usize,
) -> Option<usize> {
    if o_index == 0 && p_index == 0 {
        return Some(a);
    }
    *count += 1;

    let instruction = program[p_index];
    let literal = program[p_index + 1];

    if o_index < *min_o {
        *min_o = o_index;
    }

    if o_index == *min_o {
        if p_index < *min_p {
            *min_p = p_index;
        }

        if p_index == *min_p {
            println!(
                "P:{} => {}@{}, O:{}, A:{}, B:{}, C:{:?}",
                p_index, instruction, literal, o_index, a, b, c_option
            );
        }
    }

    if instruction == 0 {
        if literal < 4 {
            for i in 0..0b1 << literal {
                if let Some(result) = if p_index == 0 {
                    reverse(
                        program.len() - 2,
                        o_index - 1,
                        program,
                        (a << literal) | i,
                        b,
                        c_option,
                        count,
                        min_o,
                        min_p,
                    )
                } else {
                    reverse(
                        p_index - 2,
                        o_index,
                        program,
                        (a << literal) | i,
                        b,
                        c_option,
                        count,
                        min_o,
                        min_p,
                    )
                } {
                    return Some(result);
                }
            }
            return None;
        } else {
            panic!("No support for reversing division by ADV register")
        }
    }

    if instruction == 1 {
        return reverse(
            p_index - 2,
            o_index,
            program,
            a,
            b ^ literal,
            c_option,
            count,
            min_o,
            min_p,
        );
    }

    if instruction == 2 {
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            _ => panic!("Unsupported combo for CDV"),
        };

        if b != combo & 0b111 {
            return None;
        }

        if p_index == 0 {
            return reverse(
                program.len() - 2,
                o_index - 1,
                program,
                a,
                program[o_index - 1],
                c_option,
                count,
                min_o,
                min_p,
            );
        } else {
            return reverse(
                program.len() - 2,
                o_index - 1,
                program,
                a,
                program[o_index - 1],
                c_option,
                count,
                min_o,
                min_p,
            );
        }
    }

    if instruction == 3 {
        if literal != 0 {
            panic!("Unsupported literal for jump");
        }

        if a == 0 {
            return None;
        }

        return reverse(
            p_index - 2,
            o_index,
            program,
            a,
            b,
            c_option,
            count,
            min_o,
            min_p,
        );
    }

    if instruction == 4 {
        if let Some(c) = c_option {
            return reverse(
                p_index - 2,
                o_index,
                program,
                a,
                b ^ c,
                c_option,
                count,
                min_o,
                min_p,
            );
        } else {
            let depth = (program.len() - 1 - o_index) + 1;
            for new_b in 0..=(depth * 3) {
                if let Some(result) = reverse(
                    p_index - 2,
                    o_index,
                    program,
                    a,
                    new_b,
                    Some(b ^ new_b),
                    count,
                    min_o,
                    min_p,
                ) {
                    return Some(result);
                }
            }
            return None;
        }
    }

    if instruction == 5 {
        if literal == 4 {
            if a & 0b111 != program[o_index] {
                println!(
                    "Expected {} but found {} in {}",
                    program[o_index],
                    a & 0b111,
                    a
                );
                return None;
            }
            return reverse(
                p_index - 2,
                o_index,
                program,
                a,
                b,
                c_option,
                count,
                min_o,
                min_p,
            );
        } else if literal == 5 {
            if b & 0b111 != program[o_index] {
                println!(
                    "Expected {} but found {} in {}",
                    program[o_index],
                    b & 0b111,
                    b
                );
                return None;
            }
            return reverse(
                p_index - 2,
                o_index,
                program,
                a,
                b,
                c_option,
                count,
                min_o,
                min_p,
            );
        } else {
            panic!("No support for reversing output for this literal");
        }
    }

    if instruction == 7 {
        if let Some(c) = c_option {
            let combo = match literal {
                0..=3 => literal,
                4 => a,
                5 => b,
                _ => panic!("Unsupported combo for CDV"),
            };

            if c != a.checked_shr(combo as u32).unwrap_or(0) {
                return None;
            }

            return reverse(
                p_index - 2,
                o_index,
                program,
                a,
                b,
                None,
                count,
                min_o,
                min_p,
            );
        } else {
            panic!("Unset C is not supported for CDV instruction");
        }
    }

    panic!("No matching instruction");
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
