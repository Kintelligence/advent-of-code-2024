use point::Point;
use point_grid::{Direction, PointGrid};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> PointGrid<u8> {
    let mut vec = Vec::with_capacity(140 * 140);
    let mut height = 0;

    for line in input.lines() {
        for byte in line.bytes() {
            vec.push(byte);
        }

        height += 1;
    }

    PointGrid::from_vec(vec, height)
}

struct State {
    pub map: PointGrid<u8>,
    pub visited: PointGrid<bool>,
    pub queud: PointGrid<bool>,
    pub connected: Vec<(Point, Direction)>,
    pub unconnected: Vec<(Point, Direction)>,
}

struct FenceScore {
    pub current_area: usize,
    pub current_fences: usize,
    pub current_id: u8,
}

pub fn part_1(_input: &str) -> Solution {
    let grid = parse(_input);
    let height = grid.height;
    let width = grid.width;

    let mut state = State {
        map: grid,
        visited: PointGrid::empty(false, height, width),
        queud: PointGrid::empty(false, height, width),
        connected: Vec::with_capacity(height * width / 100),
        unconnected: Vec::with_capacity(height * width / 10),
    };

    let mut score = FenceScore {
        current_area: 0,
        current_fences: 0,
        current_id: 0,
    };

    state.unconnected.push((Point::new(0, 0), Direction::East));

    let mut result = 0;

    while let Some((point, from)) = state.unconnected.pop() {
        if state.visited[point] {
            continue;
        }

        score.current_id = state.map[point];
        score.current_area = 0;
        score.current_fences = 0;
        state.visited[point] = true;

        visit(point, from, &mut state, &mut score);

        while let Some((point, from)) = state.connected.pop() {
            visit(point, from, &mut state, &mut score);
        }

        let score = score.current_area * score.current_fences;
        result += score;
    }

    result.into()
}

fn visit(point: Point, from: Direction, state: &mut State, score: &mut FenceScore) {
    score.current_area += 1;

    let mut i = 0;
    for (neighbour, direction) in state.map.adjacent_three_directional(point, from) {
        let is_connected = state.map[neighbour] == score.current_id;

        if !is_connected && !state.queud[neighbour] {
            state.unconnected.push((neighbour, direction));
            state.queud[neighbour] = true;
        } else if is_connected && !state.visited[neighbour] {
            state.connected.push((neighbour, direction));
            state.visited[neighbour] = true;
        }

        if !is_connected {
            score.current_fences += 1;
        }
        i += 1;
    }

    score.current_fences += 4 - i;
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

pub fn part_2(_input: &str) -> Solution {
    let grid = parse(_input);
    let height = grid.height;
    let width = grid.width;

    let mut state = State {
        map: grid,
        visited: PointGrid::empty(false, height, width),
        queud: PointGrid::empty(false, height, width),
        connected: Vec::with_capacity(height * width / 100),
        unconnected: Vec::with_capacity(height * width / 10),
    };

    let mut score = FenceScore {
        current_area: 0,
        current_fences: 0,
        current_id: 0,
    };

    state.unconnected.push((Point::new(0, 0), Direction::East));

    let mut result = 0;

    while let Some((point, from)) = state.unconnected.pop() {
        if state.visited[point] {
            continue;
        }

        score.current_id = state.map[point];
        score.current_area = 0;
        score.current_fences = 0;
        state.visited[point] = true;

        visit_2(point, from, &mut state, &mut score);

        while let Some((point, from)) = state.connected.pop() {
            visit_2(point, from, &mut state, &mut score);
        }

        let score = score.current_area * score.current_fences;
        result += score;
    }

    result.into()
}

fn visit_2(point: Point, from: Direction, state: &mut State, score: &mut FenceScore) {
    score.current_area += 1;

    let mut i = 0;
    for (neighbour, direction) in state.map.adjacent_three_directional(point, from) {
        let is_connected = state.map[neighbour] == score.current_id;

        if !is_connected && !state.queud[neighbour] {
            state.unconnected.push((neighbour, direction));
            state.queud[neighbour] = true;
        } else if is_connected && !state.visited[neighbour] {
            state.connected.push((neighbour, direction));
            state.visited[neighbour] = true;
        }

        if !is_connected {
            score.current_fences += 1;
        }
        i += 1;
    }

    score.current_fences += 4 - i;
}
#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 140)]
    #[test_case(include_str!("_test_2.txt"), 772)]
    #[test_case(include_str!("_test_3.txt"), 1930)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
