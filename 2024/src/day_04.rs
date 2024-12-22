use utils::{grid::Grid, Solution};

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const WORD_REV: [char; 4] = ['S', 'A', 'M', 'X'];

const PATTERN: [char; 3] = ['M', 'A', 'S'];
const PATTERN_REV: [char; 3] = ['S', 'A', 'M'];

pub fn solve(input: &str) -> Solution {
    let data = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input.lines().next().unwrap().len();
    let height = data.len() / width;

    let grid = Grid::new(data, width, height);

    let mut part_1 = 0;
    for column in grid.columns() {
        part_1 += column
            .into_iter()
            .map_windows(|&bytes| bytes.map(|&byte| byte))
            .map(|bytes| (bytes == WORD) as usize + (bytes == WORD_REV) as usize)
            .sum::<usize>();
    }
    for row in grid.rows() {
        part_1 += row
            .into_iter()
            .map_windows(|&bytes| bytes.map(|&byte| byte))
            .map(|bytes| (bytes == WORD) as usize + (bytes == WORD_REV) as usize)
            .sum::<usize>();
    }
    for (column, row, _) in grid.iter() {
        'down_left: {
            let Some(byte) = get_down_left_4(&grid, column, row) else {
                break 'down_left;
            };

            part_1 += (byte == WORD) as usize + (byte == WORD_REV) as usize;
        }

        'down_right: {
            let Some(byte) = get_down_right_4(&grid, column, row) else {
                break 'down_right;
            };

            part_1 += (byte == WORD) as usize + (byte == WORD_REV) as usize;
        }
    }

    let mut part_2 = 0usize;
    for (column, row, _) in grid.iter() {
        let Some(first) = get_down_right_3(&grid, column, row) else {
            continue;
        };

        let Some(second) = get_down_left_3(&grid, column, row) else {
            continue;
        };

        part_2 += ((first == PATTERN || first == PATTERN_REV)
            && (second == PATTERN || second == PATTERN_REV)) as usize;
    }

    (part_1, part_2).into()
}

fn get_down_right_3(grid: &Grid<char>, column: usize, row: usize) -> Option<[char; 3]> {
    Some([
        grid.get(column, row).copied()?,
        grid.get(column.checked_add(1)?, row.checked_add(1)?)
            .copied()?,
        grid.get(column.checked_add(2)?, row.checked_add(2)?)
            .copied()?,
    ])
}

fn get_down_left_3(grid: &Grid<char>, column: usize, row: usize) -> Option<[char; 3]> {
    Some([
        grid.get(column.checked_add(2)?, row).copied()?,
        grid.get(column.checked_add(1)?, row.checked_add(1)?)
            .copied()?,
        grid.get(column, row.checked_add(2)?).copied()?,
    ])
}

fn get_down_right_4(grid: &Grid<char>, column: usize, row: usize) -> Option<[char; 4]> {
    Some([
        grid.get(column, row).copied()?,
        grid.get(column.checked_add(1)?, row.checked_add(1)?)
            .copied()?,
        grid.get(column.checked_add(2)?, row.checked_add(2)?)
            .copied()?,
        grid.get(column.checked_add(3)?, row.checked_add(3)?)
            .copied()?,
    ])
}

fn get_down_left_4(grid: &Grid<char>, column: usize, row: usize) -> Option<[char; 4]> {
    Some([
        grid.get(column.checked_add(3)?, row).copied()?,
        grid.get(column.checked_add(2)?, row).copied()?,
        grid.get(column.checked_add(1)?, row.checked_add(1)?)
            .copied()?,
        grid.get(column, row.checked_add(2)?).copied()?,
    ])
}
