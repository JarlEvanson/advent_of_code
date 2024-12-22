use utils::{direction::Direction, grid::Grid, hash::FxHashSet, Solution};

pub fn solve(input: &str) -> Solution {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.into_iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .into_iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let mut grid = Grid::new(data, width, height);

    let start_direction = Direction::North;
    let start_pos = 'find_guard: {
        for (column, row, &value) in grid.iter() {
            if value == b'^' {
                break 'find_guard (column as isize, row as isize);
            }
        }

        unreachable!()
    };

    let part_1_locations = {
        let mut guard_direction = start_direction;
        let mut guard_position = start_pos;

        let mut locations = FxHashSet::default();
        while let Some((new_position, new_direction)) =
            guard_step(&grid, guard_position, guard_direction)
        {
            locations.insert(guard_position);

            guard_position = new_position;
            guard_direction = new_direction;
        }

        locations.insert(guard_position);

        locations
    };

    let mut part_2 = 0usize;

    let mut visited = FxHashSet::default();
    'locations: for location in part_1_locations.iter().copied() {
        if location == start_pos {
            continue;
        }

        visited.clear();

        *grid.get_signed_mut(location.0, location.1).unwrap() = b'#';

        let mut guard_direction = start_direction;
        let mut guard_position = start_pos;
        while visited.insert((guard_position, guard_direction)) {
            let Some((new_position, new_direction)) =
                guard_step(&grid, guard_position, guard_direction)
            else {
                *grid.get_signed_mut(location.0, location.1).unwrap() = b'.';
                continue 'locations;
            };

            guard_position = new_position;
            guard_direction = new_direction;
        }

        *grid.get_signed_mut(location.0, location.1).unwrap() = b'.';

        part_2 += 1;
    }

    (part_1_locations.len(), part_2).into()
}

fn guard_step(
    grid: &Grid<u8>,
    position: (isize, isize),
    direction: Direction,
) -> Option<((isize, isize), Direction)> {
    let test_position = (
        position.0 + direction.offset().0,
        position.1 + direction.offset().1,
    );

    match grid.get_signed(test_position.0, test_position.1) {
        Some(&value) if value != b'#' => Some((test_position, direction)),
        Some(_) => Some((position, direction.clockwise())),
        None => None,
    }
}
