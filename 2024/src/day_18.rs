use std::{cmp::Reverse, collections::BinaryHeap};

use utils::{direction::Direction, grid::Grid, hash::FxHashMap, Solution};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const BYTES: usize = 1024;

pub fn solve(input: &str) -> Solution {
    let mut byte_falls = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .enumerate();

    let mut grid = Grid::new(vec![true; WIDTH * HEIGHT].into_boxed_slice(), WIDTH, HEIGHT);

    let mut part_1 = None;
    let mut part_2 = None;

    let skip_to = 0;
    while let Some((index, (column, row))) = byte_falls.next() {
        *grid.get_mut(column, row).unwrap() = false;

        if index < skip_to && index != BYTES {
            continue;
        }

        let mut graph = FxHashMap::default();
        for row in 0..grid.height() {
            for column in 0..grid.width() {
                if *grid.get(column, row).unwrap() {
                    add_neighbors(&grid, &mut graph, (column as isize, row as isize));
                }
            }
        }

        let result = solve_graph(&graph);

        if result.is_none() && part_2.is_none() {
            part_2 = Some(format!("{column},{row}"));
        }

        if index == BYTES {
            part_1 = result;
        }
    }

    (part_1.unwrap(), part_2.unwrap()).into()
}

pub fn add_neighbors(
    grid: &Grid<bool>,
    graph: &mut FxHashMap<(isize, isize), Vec<(usize, (isize, isize))>>,
    coords: (isize, isize),
) {
    let entry: &mut Vec<_> = graph.entry(coords).or_default();

    for (x_offset, y_offset) in Direction::ALL.into_iter().map(|dir| dir.offset()) {
        let x = coords.0 + x_offset;
        let y = coords.1 + y_offset;

        if grid.get_signed(x, y).copied().is_some_and(|tile| tile) {
            entry.push((1, (x, y)));
        }
    }
}

fn solve_graph(graph: &FxHashMap<(isize, isize), Vec<(usize, (isize, isize))>>) -> Option<usize> {
    let mut unvisited = BinaryHeap::default();
    unvisited.push(Reverse((0, (0, 0))));

    let distances = shortest_path(&graph, unvisited);

    distances
        .get(&(WIDTH as isize - 1, HEIGHT as isize - 1))
        .copied()
}

fn shortest_path(
    graph: &FxHashMap<(isize, isize), Vec<(usize, (isize, isize))>>,
    mut unvisited: BinaryHeap<Reverse<(usize, (isize, isize))>>,
) -> FxHashMap<(isize, isize), usize> {
    let mut visited = FxHashMap::default();

    while let Some(Reverse((distance, location))) = unvisited.pop() {
        if visited.contains_key(&location) {
            continue;
        }

        visited.insert(location, distance);

        for &(additional_distance, new_location) in graph.get(&location).unwrap() {
            unvisited.push(Reverse((distance + additional_distance, new_location)));
        }
    }

    visited
}
