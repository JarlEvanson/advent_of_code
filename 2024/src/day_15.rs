use core::str;

use utils::{direction::Direction, grid::Grid, Solution};

pub fn solve(input: &str) -> Solution {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let movements = movements
        .as_bytes()
        .into_iter()
        .filter(|&&byte| byte != b'\n')
        .map(|&byte| {
            match byte {
                b'<' => Direction::West,
                b'>' => Direction::East,
                b'^' => Direction::North,
                b'v' => Direction::South,
                _ => unreachable!(),
            }
            .offset()
        });

    (
        solve_part_1(map, movements.clone()),
        solve_part_2(map, movements),
    )
        .into()
}

fn solve_part_1(map: &str, movements: impl Iterator<Item = (isize, isize)>) -> usize {
    let data = map
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().map(|&byte| byte))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = map
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let mut map = Grid::new(data, width, height);

    let mut pos = (0, 0);
    for row in 0..map.height() {
        for column in 0..map.width() {
            let value = *map.get(column, row).unwrap();

            if value == b'@' {
                pos = (column as isize, row as isize);
            }
        }
    }

    'movement_loop: for offset in movements {
        let mut end_push_pos = pos;
        while let Some(value) =
            map.get_signed_mut(end_push_pos.0 + offset.0, end_push_pos.1 + offset.1)
        {
            if *value == b'#' {
                continue 'movement_loop;
            } else if *value == b'O' {
                end_push_pos = (end_push_pos.0 + offset.0, end_push_pos.1 + offset.1);
            } else {
                break;
            };
        }

        *map.get_signed_mut(end_push_pos.0 + offset.0, end_push_pos.1 + offset.1)
            .unwrap() = b'O';

        *map.get_signed_mut(pos.0, pos.1).unwrap() = b'.';

        pos = (pos.0 + offset.0, pos.1 + offset.1);
        *map.get_signed_mut(pos.0, pos.1).unwrap() = b'@';
    }

    let mut solution = 0;
    for row in 0..map.height() {
        for column in 0..map.width() {
            if *map.get(column, row).unwrap() == b'O' {
                solution += row * 100 + column;
            }
        }
    }

    solution
}

fn solve_part_2(map: &str, movements: impl Iterator<Item = (isize, isize)>) -> usize {
    let mut position = 0;

    let mut map_vector = Vec::with_capacity(map.len());
    for line in map.as_bytes().split(|&byte| byte == b'\n') {
        for &byte in line {
            if byte == b'#' {
                map_vector.push(b'#');
                map_vector.push(b'#');
            } else if byte == b'O' {
                map_vector.push(b'[');
                map_vector.push(b']');
            } else if byte == b'.' {
                map_vector.push(b'.');
                map_vector.push(b'.');
            } else {
                assert_eq!(byte, b'@');

                position = map_vector.len();
                map_vector.push(b'@');
                map_vector.push(b'.');
            }
        }
    }

    let width = map
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap()
        * 2;
    let height = map_vector.len() / width;

    assert_eq!(map_vector.len(), width * height);
    let mut map = Grid::new(map_vector.into_boxed_slice(), width, height);

    let mut pos = ((position % width) as isize, (position / width) as isize);
    for offset in movements {
        if check_push(&map, pos, offset) {
            push(&mut map, pos, b'@', offset);

            *map.get_signed_mut(pos.0, pos.1).unwrap() = b'.';

            pos = (pos.0 + offset.0, pos.1 + offset.1);
            *map.get_signed_mut(pos.0, pos.1).unwrap() = b'@';
        }
    }

    let mut solution = 0;
    for row in 0..map.height() {
        for column in 0..map.width() {
            if *map.get(column, row).unwrap() == b'[' {
                solution += row * 100 + column;
            }
        }
    }

    solution
}

fn check_push(map: &Grid<u8>, position: (isize, isize), offset: (isize, isize)) -> bool {
    let new_position = (position.0 + offset.0, position.1 + offset.1);
    let object = *map.get_signed(new_position.0, new_position.1).unwrap();

    match object {
        b'.' => true,
        b'#' => false,
        b'[' => {
            if offset.1 != 0 {
                check_push(map, new_position, offset)
                    && check_push(map, (new_position.0 + 1, new_position.1), offset)
            } else {
                check_push(map, new_position, offset)
            }
        }
        b']' => {
            if offset.1 != 0 {
                check_push(map, new_position, offset)
                    && check_push(map, (new_position.0 - 1, new_position.1), offset)
            } else {
                check_push(map, new_position, offset)
            }
        }

        object => unreachable!("{}", object as char),
    }
}

fn push(map: &mut Grid<u8>, position: (isize, isize), moved_object: u8, offset: (isize, isize)) {
    let new_position = (position.0 + offset.0, position.1 + offset.1);
    let value = *map.get_signed(new_position.0, new_position.1).unwrap();

    match value {
        b'.' => {}
        b'[' => {
            push(map, new_position, b'[', offset);
            if offset.1 != 0 {
                push(map, (new_position.0 + 1, new_position.1), b']', offset);
            }
        }
        b']' => {
            push(map, new_position, b']', offset);
            if offset.1 != 0 {
                push(map, (new_position.0 - 1, new_position.1), b'[', offset);
            }
        }
        value => unreachable!("{}", value as char),
    }

    *map.get_signed_mut(new_position.0, new_position.1).unwrap() = moved_object;
    *map.get_signed_mut(position.0, position.1).unwrap() = b'.';
}
