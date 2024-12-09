use std::collections::VecDeque;

use parse::ToDigit;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

enum Block {
    Empty(usize),
    File(usize, usize),
}

fn parse(input: &str) -> VecDeque<Block> {
    let mut vec = VecDeque::new();
    let mut next_id = 0;

    let mut bytes = input.bytes();

    while let Some(file_byte) = bytes.next() {
        if let Some(size) = file_byte.to_digit() {
            vec.push_back(Block::File(size as usize, next_id));
            next_id += 1;
        }

        if let Some(empty_byte) = bytes.next() {
            if let Some(size) = empty_byte.to_digit() {
                vec.push_back(Block::Empty(size as usize));
            }
        }
    }

    return vec;
}

pub fn part_1(_input: &str) -> Solution {
    let mut blocks = parse(_input);
    let mut result: usize = 0;

    let mut position = 0;
    let mut fill_id = 0;
    let mut remaining = 0;

    if let Some(back) = blocks.pop_back() {
        if let Block::File(size, id) = back {
            fill_id = id;
            remaining = size;
        }
    }

    while let Some(front) = blocks.pop_front() {
        if let Block::File(size, id) = front {
            for _ in 0..size {
                result += position * id;
                position += 1;
            }
        } else if let Block::Empty(size) = front {
            for _ in 0..size {
                result += position * fill_id;
                position += 1;
                remaining -= 1;

                if remaining == 0 {
                    if let Some(back) = blocks.pop_back() {
                        if let Block::File(size, id) = back {
                            fill_id = id;
                            remaining = size;
                        } else {
                            if let Some(back) = blocks.pop_back() {
                                if let Block::File(size, id) = back {
                                    fill_id = id;
                                    remaining = size;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for _ in 0..remaining {
        result += position * fill_id;
        position += 1;
    }

    result.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 1928)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

#[derive(Debug)]
struct File {
    id: usize,
    size: usize,
    offset: usize,
}

#[derive(Debug)]
struct Space {
    size: usize,
    offset: usize,
}

pub fn part_2(_input: &str) -> Solution {
    let blocks = parse(_input);
    let mut offset = 0;

    let mut files = Vec::new();
    let mut spaces = Vec::new();

    for block in blocks {
        if let Block::File(size, id) = block {
            files.push(File { id, size, offset });
            offset += size;
        } else if let Block::Empty(size) = block {
            spaces.push(Space { size, offset });
            offset += size;
        }
    }

    files.reverse();

    let mut result: usize = 0;

    for file in files {
        let mut found = false;
        for i in 0..spaces.len() {
            if spaces[i].offset < file.offset {
                if spaces[i].size >= file.size {
                    let offset = spaces[i].offset;
                    result += file.id * (offset * 2 + file.size - 1) * file.size / 2;

                    if spaces[i].size == file.size {
                        spaces.remove(i);
                    } else {
                        spaces[i].size -= file.size;
                        spaces[i].offset += file.size;
                    }

                    found = true;
                    break;
                }
            } else {
                break;
            }
        }

        if !found {
            result += file.id * (file.offset * 2 + file.size - 1) * file.size / 2;
        }
    }

    result.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 2858)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(6304576012713)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
