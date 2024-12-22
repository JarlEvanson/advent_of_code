use utils::{direction::Direction, grid::Grid, hash::FxHashSet, Solution};

pub fn solve(input: &str) -> Solution {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().map(|&byte| byte - b'0'))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut reached_tops = FxHashSet::default();
    for row in 0..grid.height() {
        for column in 0..grid.width() {
            let value = *grid.get(column, row).unwrap();

            if value == 0 {
                reached_tops.clear();
                part_2 += explore(&grid, &mut reached_tops, (column as isize, row as isize));
                part_1 += reached_tops.len();
            }
        }
    }

    (part_1, part_2).into()
}

fn explore(
    grid: &Grid<u8>,
    reached: &mut FxHashSet<(isize, isize)>,
    position: (isize, isize),
) -> usize {
    let height = *grid.get_signed(position.0, position.1).unwrap();

    if height == 9 {
        reached.insert(position);

        return 1;
    }

    let mut score = 0;
    for (x_offset, y_offset) in Direction::ALL
        .into_iter()
        .map(|direction| direction.offset())
    {
        let column = position.0 + x_offset;
        let row = position.1 + y_offset;

        if grid
            .get_signed(column, row)
            .is_some_and(|&test_height| test_height == height + 1)
        {
            score += explore(grid, reached, (column, row));
        }
    }

    score
}
