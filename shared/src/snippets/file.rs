fn graph(
    map: &Grid<bool>,
    point: Point,
    id: usize,
    current_direction: Direction,
    visited: &mut Vec<bool>,
    translator: &mut Translator<Point>,
    nodes: &mut Vec<Node>,
    connections: &mut Vec<Connection>,
    start_position: Point,
    end_position: Point,
) {
    if visited[id] {
        return;
    }

    visited[id] = true;

    for travel_direction in current_direction.reverse().other_cardinals() {
        if nodes[id].paths.iter().any(|path| {
            let connection = &connections[*path];
            if connection.a_id == id {
                return connection.a_direction == travel_direction;
            }
            return connection.b_direction == travel_direction;
        }) {
            continue;
        }
        if let Some((next, cost, next_direction, length)) =
            travel_to_next_junction(map, point, travel_direction, start_position, end_position)
        {
            if next == point {
                continue;
            }

            let next_id = translator.translate(next);
            let next_connection_id = connections.len();

            connections.push(Connection {
                a_direction: travel_direction,
                a_id: id,
                b_direction: next_direction.reverse(),
                b_id: next_id,
                cost,
                length,
            });

            nodes[id].paths.push(next_connection_id);

            if nodes.len() == next_id {
                nodes.push(Node {
                    paths: vec![next_connection_id],
                });
            } else {
                nodes[next_id].paths.push(next_connection_id);
            }

            graph(
                map,
                next,
                next_id,
                next_direction,
                visited,
                translator,
                nodes,
                connections,
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
) -> Option<(Point, usize, Direction, usize)> {
    let mut cost = 0;
    let mut points = 0;
    if let Some(next) = map.go_if_true(current_point, current_direction) {
        current_point = next;
        loop {
            if current_point == start_position || current_point == end_position {
                return Some((current_point, cost + 1, current_direction, points));
            }

            let mut next: Option<(Point, Direction)> = None;
            for next_direction in current_direction.reverse().other_cardinals() {
                if let Some(next_point) = map.go_if_true(current_point, next_direction) {
                    if next.is_none() {
                        next = Some((next_point, next_direction));
                    } else {
                        return Some((current_point, cost + 1, current_direction, points));
                    }
                }
            }

            if let Some((next_point, next_direction)) = next {
                cost += 1;
                if current_direction != next_direction {
                    cost += 1000;
                }
                points += 1;
                current_point = next_point;
                current_direction = next_direction;
            } else {
                return None;
            }
        }
    }
    None
}

fn parse_and_graph(input: &str) -> (usize, usize, Vec<Node>, Vec<Connection>) {
    let (map, start_point, end_point) = parse(input);
    let mut translator = Translator::new();
    let mut visited = vec![false; map.height * map.width];
    let start = translator.translate(start_point);
    let start_node = Node { paths: Vec::new() };

    let mut nodes = vec![start_node];
    let mut connections = Vec::new();

    graph(
        &map,
        start_point,
        start,
        Direction::East,
        &mut visited,
        &mut translator,
        &mut nodes,
        &mut connections,
        start_point,
        end_point,
    );

    //println!("{}", map.print_bool());

    let mut map: Grid<Option<u8>> = map.same_size_with(None);
    for (&point, &node) in translator.map.iter() {
        map[point] = Some(node as u8);
    }

    //println!("{}", map.print_option_u8(1));

    let end = translator.translate(end_point);
    (start, end, nodes, connections)
}
