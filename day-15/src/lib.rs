use grid::Grid;
use points::{directions::Direction, point::Point};
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxRight,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Wall => "#",
                Tile::Box => "[",
                Tile::BoxRight => "]",
            }
        )
    }
}

fn parse(input: &str) -> (Grid<Tile>, Point, Vec<Direction>) {
    let mut vec = Vec::new();
    let mut y = 0;
    let mut start = Point::new(0, 0);
    let mut directions = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        for (x, byte) in line.bytes().enumerate() {
            if byte == b'@' {
                start = Point::new(x, y);
            }

            vec.push(match byte {
                b'#' => Tile::Wall,
                b'O' => Tile::Box,
                _ => Tile::Empty,
            });
        }

        y += 1;
    }

    while let Some(line) = lines.next() {
        for byte in line.bytes() {
            directions.push(match byte {
                b'^' => Direction::North,
                b'>' => Direction::East,
                b'v' => Direction::South,
                b'<' => Direction::West,
                _ => panic!("Unexpected direction"),
            });
        }
    }

    (Grid::from(vec, y), start, directions)
}

fn move_small_boxes(map: &mut Grid<Tile>, point: Point, direction: Direction) -> Point {
    if let Some(destination) = map.go(point, direction) {
        if map[destination] == Tile::Empty {
            return destination;
        }

        if map[destination] == Tile::Box {
            let mut current = destination;
            while let Some(next) = map.go(current, direction) {
                if map[next] == Tile::Empty {
                    map[next] = Tile::Box;
                    map[destination] = Tile::Empty;
                    return destination;
                }

                if map[next] == Tile::Wall {
                    return point;
                }

                current = next;
            }
        }
    }

    point
}

pub fn part_1(_input: &str) -> Solution {
    let (mut map, start, directions) = parse(_input);
    let mut current = start;

    for direction in directions {
        current = move_small_boxes(&mut map, current, direction)
    }

    map.points()
        .filter_map(|p| {
            if map[p] == Tile::Box {
                Some(p.y * 100 + p.x)
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_1.txt"), 2028)]
    #[test_case(include_str!("_test_2.txt"), 10092)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn expand_map(map: Grid<Tile>) -> Grid<Tile> {
    let mut vec = Vec::with_capacity(map.height * map.width * 2);
    for tile in map.vec {
        match tile {
            Tile::Empty => {
                vec.push(Tile::Empty);
                vec.push(Tile::Empty);
            }
            Tile::Wall => {
                vec.push(Tile::Wall);
                vec.push(Tile::Wall);
            }
            Tile::Box => {
                vec.push(Tile::Box);
                vec.push(Tile::BoxRight);
            }
            _ => panic!("Unexpected right box in unexpanded map"),
        }
    }

    Grid::from(vec, map.height)
}

fn move_next_box(map: &mut Grid<Tile>, from: Point, to: Point) {
    let from_tile = map[from];

    if from_tile == Tile::Empty {
        return;
    }

    let (other_from, other_to) = if from_tile == Tile::Box {
        (from.right(), to.right())
    } else {
        (from.left(), to.left())
    };
    let other_from_tile = map[other_from];

    map[to] = from_tile;
    map[from] = Tile::Empty;
    map[other_to] = other_from_tile;
    map[other_from] = Tile::Empty;
}

fn try_move_big_boxes(
    map: &mut Grid<Tile>,
    point: Point,
    direction: Direction,
    move_queue: &mut Vec<(Point, Point, usize)>,
    depth: usize,
) -> Option<Point> {
    if let Some(target) = map.go(point, direction) {
        let tile = map[target];

        if tile == Tile::Empty {
            return Some(target);
        }
        if direction.is_vertical() {
            let mut box_parts: Option<(Point, Point)> = None;

            if tile == Tile::Box {
                box_parts = Some((target, target.right()));
            } else if tile == Tile::BoxRight {
                box_parts = Some((target.left(), target));
            }

            if let Some(next_target) = map.go(target, direction) {
                if tile == map[next_target] {
                    if let Some(to) =
                        try_move_big_boxes(map, target, direction, move_queue, depth + 1)
                    {
                        move_queue.push((target, to, depth));
                        return Some(target);
                    } else {
                        return None;
                    }
                }

                if let Some((left_from, right_from)) = box_parts {
                    if let Some((left_to, right_to)) =
                        try_move_big_boxes(map, left_from, direction, move_queue, depth + 1).zip(
                            try_move_big_boxes(map, right_from, direction, move_queue, depth + 1),
                        )
                    {
                        move_queue.push((left_from, left_to, depth));
                        move_queue.push((right_from, right_to, depth));
                        return Some(target);
                    }
                }
            }
        } else {
            if tile == Tile::Box || tile == Tile::BoxRight {
                if let Some(to) =
                    try_move_big_boxes(map, target.go(direction), direction, move_queue, depth + 1)
                {
                    move_queue.push((target.go(direction), to, depth));
                    return Some(target);
                }
            }
        }
    }

    None
}

pub fn part_2(_input: &str) -> Solution {
    let (small_map, start, directions) = parse(_input);
    let mut map = expand_map(small_map);
    let mut current = Point::new(start.x * 2, start.y);
    let mut move_queue = Vec::with_capacity(500);

    for direction in directions {
        if let Some(next) = try_move_big_boxes(&mut map, current, direction, &mut move_queue, 0) {
            move_queue.sort_by_key(|c| c.2);
            for &(from, to, _) in move_queue.iter().rev() {
                move_next_box(&mut map, from, to);
            }
            current = next;
        }
        move_queue.clear();
    }

    map.points()
        .filter_map(|p| {
            if map[p] == Tile::Box {
                Some(p.y * 100 + p.x)
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test_2.txt"), 9021)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1463160)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
