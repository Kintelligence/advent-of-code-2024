use parse::ToDigit;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

enum Block {
    Empty(usize),
    File(usize, usize),
}

fn parse(input: &str) -> Vec<Block> {
    let mut vec = Vec::new();
    let mut next_id = 0;

    let mut bytes = input.bytes();

    while let Some(file_byte) = bytes.next() {
        if let Some(size) = file_byte.to_digit() {
            vec.push(Block::File(size as usize, next_id));
            next_id += 1;
        }

        if let Some(empty_byte) = bytes.next() {
            if let Some(size) = empty_byte.to_digit() {
                vec.push(Block::Empty(size as usize));
            }
        }
    }

    return vec;
}

pub fn part_1(_input: &str) -> Solution {
    let blocks = parse(_input);
    let mut result: usize = 0;

    let mut back_i = blocks.len() - 1;

    let mut position = 0;
    let mut fill_id = 0;
    let mut remaining = 0;

    if let Block::File(size, id) = blocks[back_i] {
        fill_id = id;
        remaining = size;
    }

    for i in 0..blocks.len() {
        if i >= back_i {
            break;
        }

        let front = &blocks[i];
        if let Block::File(size, id) = front {
            result += id * (position * 2 + size - 1) * size / 2;
            position += size;
        } else if let Block::Empty(size) = front {
            let mut hole = *size;

            while hole > 0 {
                let min = usize::min(hole, remaining);
                hole -= min;
                remaining -= min;
                result += fill_id * (position * 2 + min - 1) * min / 2;
                position += min;

                if remaining == 0 {
                    back_i -= 2;
                    if back_i <= i {
                        break;
                    }
                    if let Block::File(size, id) = blocks[back_i] {
                        fill_id = id;
                        remaining = size;
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

    #[test_case(6283404590840)]
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

    let mut result: usize = 0;
    let mut cache: [usize; 10] = [0; 10];

    for file in files.iter().rev() {
        let mut found = false;
        for i in cache[file.size]..file.id {
            if spaces[i].size >= file.size {
                let offset = spaces[i].offset;
                result += file.id * (offset * 2 + file.size - 1) * file.size / 2;
                spaces[i].size -= file.size;
                spaces[i].offset += file.size;
                cache[file.size] = i;
                found = true;
                break;
            }
        }

        if !found {
            result += file.id * (file.offset * 2 + file.size - 1) * file.size / 2;
            cache[file.size] = usize::MAX;
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
