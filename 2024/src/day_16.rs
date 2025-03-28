use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap},
};

use utils::{
    direction::Direction,
    grid::Grid,
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().map(|&byte| byte))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let (start_pos, end_pos) = 'positions_loop: {
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        for (column, row, &value) in grid.iter() {
            if value == b'S' {
                start_pos = (column as isize, row as isize);
                if end_pos != (0, 0) {
                    break 'positions_loop (start_pos, end_pos);
                }
            } else if value == b'E' {
                end_pos = (column as isize, row as isize);
                if start_pos != (0, 0) {
                    break 'positions_loop (start_pos, end_pos);
                }
            }
        }

        unreachable!()
    };

    let mut graph = FxHashMap::default();
    for (column, row, &value) in grid.iter() {
        if value == b'#' {
            continue;
        }

        add_neighbors(&grid, &mut graph, (column as isize, row as isize));
    }

    let (part_1, part_2) = solve_graph(&graph, start_pos, end_pos);

    (part_1, part_2).into()
}

fn add_neighbors(
    grid: &Grid<u8>,
    graph: &mut FxHashMap<(isize, isize), Vec<(usize, (isize, isize), Direction)>>,
    coords: (isize, isize),
) {
    let entry: &mut Vec<_> = graph.entry(coords).or_default();

    for (dir, (x_offset, y_offset)) in Direction::ALL.into_iter().map(|dir| (dir, dir.offset())) {
        let x = coords.0 + x_offset;
        let y = coords.1 + y_offset;

        if grid
            .get_signed(x, y)
            .copied()
            .is_some_and(|tile| tile != b'#')
        {
            entry.push((1, (x, y), dir));
        }
    }
}

fn solve_graph(
    graph: &FxHashMap<(isize, isize), Vec<(usize, (isize, isize), Direction)>>,
    start: (isize, isize),
    end: (isize, isize),
) -> (usize, usize) {
    let mut unvisited = BinaryHeap::new();
    unvisited.push(Reverse((0, start, Direction::East, None)));

    let distances = shortest_path(graph, unvisited);

    let shortest_distance = Direction::ALL
        .into_iter()
        .flat_map(|direction| {
            distances
                .get(&(end, direction))
                .map(|&(distance, _)| distance)
        })
        .min()
        .unwrap();

    let mut prev = Vec::default();
    let mut visited = FxHashSet::default();

    let valid_ends = Direction::ALL.into_iter().filter_map(|direction| {
        let Some(&distance) = distances
            .get(&(end, direction))
            .map(|(distance, _)| distance)
        else {
            return None;
        };

        if distance != shortest_distance {
            return None;
        }

        Some((end, direction))
    });

    for (prev_node, prev_direction) in valid_ends {
        prev.push((prev_node, prev_direction));
    }

    while let Some((prev_node, prev_direction)) = prev.pop() {
        visited.insert(prev_node);

        let (_, prev_nodes) = distances.get(&(prev_node, prev_direction)).unwrap();
        for prev_node in prev_nodes.iter().copied() {
            prev.push(prev_node);
        }
    }

    (shortest_distance, visited.len())
}

fn shortest_path(
    graph: &FxHashMap<(isize, isize), Vec<(usize, (isize, isize), Direction)>>,
    mut unvisited: BinaryHeap<
        Reverse<(
            usize,
            (isize, isize),
            Direction,
            Option<((isize, isize), Direction)>,
        )>,
    >,
) -> FxHashMap<((isize, isize), Direction), (usize, Vec<((isize, isize), Direction)>)> {
    let mut visited = FxHashMap::<
        ((isize, isize), Direction),
        (usize, Vec<((isize, isize), Direction)>),
    >::default();

    while let Some(Reverse((distance, location, direction, prev))) = unvisited.pop() {
        match visited.entry((location, direction)) {
            Entry::Occupied(mut occupied) => {
                if distance != occupied.get().0 {
                    continue;
                }

                let Some(prev) = prev else {
                    continue;
                };

                occupied.get_mut().1.push(prev);
                continue;
            }
            Entry::Vacant(vacant) => {
                let prevs = if let Some(prev) = prev {
                    vec![prev]
                } else {
                    Vec::new()
                };

                vacant.insert((distance, prevs));
            }
        }

        let neighbors = graph.get(&(location)).unwrap();
        for &(test_distance, test_location, test_direction) in neighbors {
            if test_direction == direction {
                unvisited.push(Reverse((
                    distance + test_distance,
                    test_location,
                    test_direction,
                    Some((location, direction)),
                )));
            } else if test_direction == direction.clockwise()
                || test_direction == direction.counterclockwise()
            {
                unvisited.push(Reverse((
                    distance + test_distance + 1000,
                    test_location,
                    test_direction,
                    Some((location, direction)),
                )));
            }
        }
    }

    visited
}
