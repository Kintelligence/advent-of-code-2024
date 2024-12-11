use grid::{Directions, Grid, DIRECTIONS};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

fn parse(input: &str) -> Grid<Letter> {
    let lines = input.lines();
    let mut height = 0;
    let mut vec = Vec::new();

    for line in lines {
        height += 1;
        for char in line.bytes() {
            match char {
                b'X' => vec.push(Letter::X),
                b'M' => vec.push(Letter::M),
                b'A' => vec.push(Letter::A),
                b'S' => vec.push(Letter::S),
                _ => {
                    panic!("Unexpected character")
                }
            }
        }
    }

    Grid::from_vec(vec, height)
}

fn test_1(map: &Grid<Letter>, x: usize, y: usize, direction: Directions) -> bool {
    if let Some((x, y)) = map.go(x, y, direction) {
        if *map.index(x, y) == Letter::M {
            if let Some((x, y)) = map.go(x, y, direction) {
                if *map.index(x, y) == Letter::A {
                    if let Some((x, y)) = map.go(x, y, direction) {
                        if *map.index(x, y) == Letter::S {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn solve_1(map: Grid<Letter>) -> usize {
    let mut count = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if *map.index(x, y) == Letter::X {
                for direction in DIRECTIONS {
                    if test_1(&map, x, y, direction) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);
    solve_1(map).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 18)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(2543)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn test_2(
    map: &Grid<Letter>,
    x: usize,
    y: usize,
    direction: Directions,
    m: &mut bool,
    s: &mut bool,
) {
    if let Some((x, y)) = map.go(x, y, direction) {
        let letter = map.index(x, y);
        if *letter == Letter::M {
            *m = true;
        }
        if *letter == Letter::S {
            *s = true;
        }
    }
}

fn solve_2(map: Grid<Letter>) -> usize {
    let mut count = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if *map.index(x, y) == Letter::A {
                let mut m_1 = false;
                let mut s_1 = false;
                let mut m_2 = false;
                let mut s_2 = false;

                test_2(&map, x, y, Directions::NorthEast, &mut m_1, &mut s_1);
                if !m_1 && !s_1 {
                    continue;
                }
                test_2(&map, x, y, Directions::SouthWest, &mut m_1, &mut s_1);
                if !m_1 || !s_1 {
                    continue;
                }
                test_2(&map, x, y, Directions::NorthWest, &mut m_2, &mut s_2);
                if !m_2 && !s_2 {
                    continue;
                }
                test_2(&map, x, y, Directions::SouthEast, &mut m_2, &mut s_2);
                if !m_2 || !s_2 {
                    continue;
                }

                count += 1;
            }
        }
    }

    count
}

pub fn part_2(_input: &str) -> Solution {
    let map = parse(_input);
    solve_2(map).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 9)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1930)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
