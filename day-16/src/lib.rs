use grid::Grid;
use points::{
    directions::{Direction, CARDINALS},
    point::Point,
};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

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

pub fn part_1(_input: &str) -> Solution {
    let (map, start, end) = parse(_input);
    let mut costs = map.same_size_with([usize::MAX; 4]);
    let mut buckets: Vec<Vec<(Point, Direction, usize)>> = vec![Vec::new(); 1001];
    let mut bucket = 0;
    buckets[0].push((start, Direction::East, 0));
    costs[start][Direction::East as usize] = 0;

    loop {
        while let Some((point, direction, cost)) = buckets[bucket % 1001].pop() {
            if point == end {
                return cost.into();
            }

            let options = [
                (map.go_if_true(point, direction), direction, cost + 1),
                (Some(point), direction.rotate_counter_90(), cost + 1000),
                (Some(point), direction.rotate_90(), cost + 1000),
            ];

            for (point_option, direction, cost) in options {
                if let Some(point) = point_option {
                    if cost < costs[point][direction as usize] {
                        costs[point][direction as usize] = cost;
                        buckets[cost % 1001].push((point, direction, cost));
                    }
                }
            }
        }
        bucket += 1;
    }
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
    let (map, start, end) = parse(_input);
    let mut costs = map.same_size_with([usize::MAX; 4]);
    let mut buckets: Vec<Vec<(Point, Direction, usize)>> = vec![Vec::new(); 1001];
    let mut bucket = 0;
    buckets[0].push((start, Direction::East, 0));
    costs[start][Direction::East as usize] = 0;
    let mut lowest: Option<usize> = None;

    'traversal: loop {
        while let Some((point, direction, cost)) = buckets[bucket % 1001].pop() {
            if point == end {
                if let Some(lowest) = lowest {
                    if cost > lowest {
                        break 'traversal;
                    }
                } else {
                    lowest = Some(cost);
                }
            }

            let options = [
                (map.go_if_true(point, direction), direction, cost + 1),
                (Some(point), direction.rotate_counter_90(), cost + 1000),
                (Some(point), direction.rotate_90(), cost + 1000),
            ];

            for (point_option, direction, cost) in options {
                if let Some(point) = point_option {
                    if cost < costs[point][direction as usize] {
                        costs[point][direction as usize] = cost;
                        buckets[cost % 1001].push((point, direction, cost));
                    }
                }
            }
        }
        bucket += 1;
    }

    let mut visited = map.same_size_with(false);
    visited[end] = true;
    if let Some(lowest) = lowest {
        let mut queue: Vec<(Point, Direction, usize)> = Vec::new();
        for direction in CARDINALS {
            if costs[end][direction as usize] == lowest {
                queue.push((end, direction, lowest));
            }
        }

        while let Some((point, from_direction, remaining)) = queue.pop() {
            let options = [
                (
                    map.go_if_true(point, from_direction.reverse()),
                    from_direction,
                    remaining - 1,
                ),
                (
                    Some(point),
                    from_direction.rotate_counter_90(),
                    remaining - 1000,
                ),
                (Some(point), from_direction.rotate_90(), remaining - 1000),
            ];

            for (point_option, from_direction, remaining) in options {
                if let Some(point) = point_option {
                    if costs[point][from_direction as usize] == remaining {
                        visited[point] = true;
                        queue.push((point, from_direction, remaining));
                    }
                }
            }
        }
    }

    visited.vec.iter().filter(|&p| *p).count().into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 45)]
    #[test_case(include_str!("_test_2.txt"), 64)]
    #[test_case(include_str!("_test_4.txt"), 12)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(527)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
