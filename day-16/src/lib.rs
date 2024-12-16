use fxhash::FxHashSet as HashSet;
use grid::Grid;
use misc::translator::Translator;
use points::{directions::Direction, point::Point};
use shared::*;
use std::collections::BinaryHeap;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Debug)]
struct Node {
    paths: Vec<Path>,
}

#[derive(Debug)]
struct Path {
    start_direction: Direction,
    end_direction: Direction,
    destination: usize,
    cost: usize,
    visited: Vec<Point>,
}

fn parse(input: &str) -> (Grid<bool>, Point, Point) {
    let mut vec = Vec::new();
    let mut y = 0;

    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for line in input.lines() {
        for (x, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => vec.push(false),
                b'.' => vec.push(true),
                b'S' => {
                    vec.push(true);
                    start = Point::new(x, y);
                }
                b'E' => {
                    vec.push(true);
                    end = Point::new(x, y);
                }
                _ => panic!("Unexpected input"),
            }
        }
        y += 1;
    }

    return (Grid::from(vec, y), start, end);
}

fn graph(
    map: &Grid<bool>,
    point: Point,
    id: usize,
    current_direction: Direction,
    visited: &mut Vec<bool>,
    translator: &mut Translator<Point>,
    nodes: &mut Vec<Node>,
    start_position: Point,
    end_position: Point,
) {
    if visited[id] {
        return;
    }

    visited[id] = true;

    for travel_direction in current_direction.reverse().other_cardinals() {
        if nodes[id]
            .paths
            .iter()
            .any(|path| path.start_direction == travel_direction)
        {
            continue;
        }
        if let Some((next, cost, next_direction, points)) =
            travel_to_next_junction(map, point, travel_direction, start_position, end_position)
        {
            if next == point {
                continue;
            }

            let next_id = translator.translate(next);

            nodes[id].paths.push(Path {
                start_direction: travel_direction,
                end_direction: next_direction,
                destination: next_id,
                cost,
                visited: points.clone(),
            });

            let reverse_path = Path {
                start_direction: next_direction.reverse(),
                end_direction: travel_direction.reverse(),
                destination: id,
                cost,
                visited: points,
            };

            if nodes.len() == next_id {
                nodes.push(Node {
                    paths: vec![reverse_path],
                });
            } else {
                nodes[next_id].paths.push(reverse_path);
            }

            graph(
                map,
                next,
                next_id,
                next_direction,
                visited,
                translator,
                nodes,
                start_position,
                end_position,
            );
        }
    }
}

fn travel_to_next_junction(
    map: &Grid<bool>,
    mut current_point: Point,
    mut current_direction: Direction,
    start_position: Point,
    end_position: Point,
) -> Option<(Point, usize, Direction, Vec<Point>)> {
    let mut cost = 0;
    let mut points = vec![current_point];
    if let Some(next) = map.go_if_true(current_point, current_direction) {
        current_point = next;
        loop {
            if current_point == start_position || current_point == end_position {
                points.push(current_point);
                return Some((current_point, cost + 1, current_direction, points));
            }

            let mut next: Option<(Point, Direction)> = None;
            for next_direction in current_direction.reverse().other_cardinals() {
                if let Some(next_point) = map.go_if_true(current_point, next_direction) {
                    if next.is_none() {
                        next = Some((next_point, next_direction));
                    } else {
                        points.push(current_point);
                        return Some((current_point, cost + 1, current_direction, points));
                    }
                }
            }

            if let Some((next_point, next_direction)) = next {
                cost += 1;
                if current_direction != next_direction {
                    points.push(current_point);
                    cost += 1000;
                }

                current_point = next_point;
                current_direction = next_direction;
            } else {
                return None;
            }
        }
    }
    None
}

fn parse_and_graph(input: &str) -> (usize, usize, Vec<Node>) {
    let (map, start_point, end_point) = parse(input);
    let mut translator = Translator::new();
    let mut visited = vec![false; map.height * map.width];
    let start = translator.translate(start_point);
    let start_node = Node { paths: Vec::new() };
    let mut nodes = vec![start_node];
    graph(
        &map,
        start_point,
        start,
        Direction::East,
        &mut visited,
        &mut translator,
        &mut nodes,
        start_point,
        end_point,
    );

    let end = translator.translate(end_point);
    (start, end, nodes)
}

pub fn part_1(_input: &str) -> Solution {
    let (start, end, nodes) = parse_and_graph(_input);
    find_shortest_path(start, end, &nodes).into()
}

#[derive(PartialEq, Eq)]
struct State {
    id: usize,
    cost: usize,
    direction: Direction,
}

impl State {
    pub fn new(id: usize, cost: usize, direction: Direction) -> Self {
        State {
            id,
            cost,
            direction,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_shortest_path(start: usize, end: usize, nodes: &Vec<Node>) -> usize {
    let mut costs = vec![usize::MAX; nodes.len()];
    let mut queue = BinaryHeap::new();

    costs[start] = 0;
    queue.push(State::new(start, 0, Direction::East));

    while let Some(state) = queue.pop() {
        if state.id == end {
            return state.cost;
        }

        if costs[state.id] < state.cost {
            continue;
        }

        for path in nodes[state.id].paths.iter() {
            if path.start_direction == state.direction.reverse() {
                continue;
            }

            let mut path_cost = state.cost + path.cost;
            if path.start_direction != state.direction {
                path_cost += 1000;
            }

            if path_cost >= costs[path.destination] {
                continue;
            }
            costs[state.id] = path_cost;
            queue.push(State::new(path.destination, path_cost, path.end_direction));
        }
    }

    usize::MAX
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 7036)]
    #[test_case(include_str!("_test_2.txt"), 11048)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(102460)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (start, end, nodes) = parse_and_graph(_input);
    fill_all_shortest_paths(start, end, &nodes).into()
}

#[derive(Debug, PartialEq, Eq)]
struct VisitState {
    cost: usize,
    id: usize,
    direction: Direction,
    visited: Vec<Vec<Point>>,
}

impl VisitState {
    pub fn new(id: usize, cost: usize, direction: Direction, visited: Vec<Vec<Point>>) -> Self {
        Self {
            id,
            cost,
            direction,
            visited,
        }
    }
}

impl Ord for VisitState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for VisitState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn fill_all_shortest_paths(start: usize, end: usize, nodes: &Vec<Node>) -> usize {
    let mut costs = vec![usize::MAX; nodes.len()];
    let mut queue = BinaryHeap::new();
    let mut visited: HashSet<(Point, Point)> = HashSet::default();

    costs[start] = 0;
    queue.push(VisitState::new(start, 0, Direction::East, Vec::new()));
    let mut lowest_cost: Option<usize> = None;

    while let Some(state) = queue.pop() {
        if state.id == end {
            if let Some(cost) = lowest_cost {
                if cost < state.cost {
                    break;
                }
            } else {
                lowest_cost = Some(state.cost);
            }

            for segments in state.visited.iter() {
                for pair in segments.windows(2) {
                    visited.insert(if pair[0] > pair[1] {
                        (pair[0], pair[1])
                    } else {
                        (pair[1], pair[0])
                    });
                }
            }
        }

        if costs[state.id] < state.cost {
            continue;
        }

        for path in nodes[state.id].paths.iter() {
            if path.start_direction == state.direction.reverse() {
                continue;
            }

            let mut path_cost = state.cost + path.cost;
            if path.start_direction != state.direction {
                path_cost += 1000;
            }

            if path_cost > costs[path.destination] {
                continue;
            }
            costs[state.id] = path_cost;
            let mut visited = state.visited.clone();
            visited.push(path.visited.clone());
            queue.push(VisitState::new(
                path.destination,
                path_cost,
                path.end_direction,
                visited,
            ));
        }
    }

    let mut visited_edges = HashSet::default();

    let mut result = 0;
    for (from, to) in visited {
        if !visited_edges.contains(&from) {
            visited_edges.insert(from);
            result += 1;
        }
        if !visited_edges.contains(&to) {
            visited_edges.insert(to);
            result += 1;
        }
        result += from.distance_to(to) - 1;
    }

    result
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 45)]
    #[test_case(include_str!("_test_2.txt"), 64)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(527)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
