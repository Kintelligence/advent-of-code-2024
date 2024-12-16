use grid::Grid;
use points::{directions::Direction, point::Point};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Grid<u8> {
    let mut vec = Vec::with_capacity(140 * 140);
    let mut height = 0;

    for line in input.lines() {
        for byte in line.bytes() {
            vec.push(byte);
        }

        height += 1;
    }

    Grid::from(vec, height)
}

struct State {
    pub map: Grid<u8>,
    pub visited: Grid<bool>,
}

struct FenceScore {
    pub current_area: usize,
    pub multiplier: usize,
    pub current_id: u8,
}

fn solve<F>(input: &str, visit: F) -> usize
where
    F: Fn(Point, &mut State, &mut FenceScore),
{
    let grid = parse(input);
    let height = grid.height;
    let width = grid.width;

    let mut state = State {
        map: grid,
        visited: Grid::filled(false, height, width),
    };

    let mut score = FenceScore {
        current_area: 0,
        multiplier: 0,
        current_id: 0,
    };

    let mut result = 0;

    for point in state.map.points() {
        if state.visited[point] {
            continue;
        }

        score.current_id = state.map[point];
        score.current_area = 0;
        score.multiplier = 0;
        state.visited[point] = true;

        visit(point, &mut state, &mut score);

        let score = score.current_area * score.multiplier;
        result += score;
    }

    result.into()
}

pub fn part_1(_input: &str) -> Solution {
    solve(_input, initial_fill).into()
}

fn initial_fill(point: Point, state: &mut State, score: &mut FenceScore) {
    fill(state.map.adjacent_four_directional(point), state, score);
}

fn fill<I>(iter: I, state: &mut State, score: &mut FenceScore)
where
    I: IntoIterator<Item = (Point, Direction)>,
{
    score.current_area += 1;
    score.multiplier += 4;

    for (neighbour, direction) in iter {
        let is_connected = state.map[neighbour] == score.current_id;

        if is_connected {
            score.multiplier -= 1;
            if !state.visited[neighbour] {
                state.visited[neighbour] = true;
                fill(
                    state.map.adjacent_three_in_direction(neighbour, direction.reverse()),
                    state,
                    score,
                );
            }
        }
    }
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 140)]
    #[test_case(include_str!("_test_2.txt"), 772)]
    #[test_case(include_str!("_test_3.txt"), 1930)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(1450422)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn initial_fill_discounted(point: Point, state: &mut State, score: &mut FenceScore) {
    fill_discounted(
        state.map.adjacent_four_directional(point),
        point,
        state,
        score,
    );
}

fn fill_discounted<I>(iter: I, point: Point, state: &mut State, score: &mut FenceScore)
where
    I: IntoIterator<Item = (Point, Direction)>,
{
    score.current_area += 1;

    let mut connected: u8 = 0b00011111;

    for (neighbour, direction) in iter {
        let is_connected = state.map[neighbour] == score.current_id;

        if is_connected {
            match direction {
                Direction::North => connected &= 0b00011110,
                Direction::East => connected &= 0b00011101,
                Direction::South => connected &= 0b00011011,
                Direction::West => connected &= 0b00010111,
                _ => {}
            }

            if !state.visited[neighbour] {
                state.visited[neighbour] = true;
                fill_discounted(
                    state.map.adjacent_four_directional(neighbour),
                    neighbour,
                    state,
                    score,
                );
            }
        }
    }

    if connected & 0b00000011 == 0b00000011 {
        score.multiplier += 1;
    }

    if connected & 0b00000110 == 0b00000110 {
        score.multiplier += 1;
    }

    if connected & 0b00001100 == 0b00001100 {
        score.multiplier += 1;
    }

    if connected & 0b00001001 == 0b00001001 {
        score.multiplier += 1;
    }

    if let Some(diagonal) = state.map.down_left(point) {
        if state.map[diagonal] == score.current_id {
            if connected & 0b00001100 == 0b00000100 {
                score.multiplier += 1;
            }

            if connected & 0b00001100 == 0b00001000 {
                score.multiplier += 1;
            }
        }
    }

    if let Some(diagonal) = state.map.down_right(point) {
        if state.map[diagonal] == score.current_id {
            if connected & 0b00000110 == 0b00000100 {
                score.multiplier += 1;
            }

            if connected & 0b00000110 == 0b00000010 {
                score.multiplier += 1;
            }
        }
    }
}

pub fn part_2(_input: &str) -> Solution {
    solve(_input, initial_fill_discounted).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 80)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(906606)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
