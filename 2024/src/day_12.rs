use utils::{direction::Direction, grid::Grid, hash::FxHashMap, unionfind::UnionFind, Solution};

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

    let mut regions = UnionFind::new(width * height);
    let mut perimeters = vec![0; width * height];
    let mut chars = vec![0; width * height];
    for (row_index, row) in grid.rows().enumerate() {
        for (column_index, val) in row.into_iter().copied().enumerate() {
            let combined_index = column_index + row_index * width;

            let mut perimeter = 0;
            for (x_offset, y_offset) in Direction::ALL
                .into_iter()
                .map(|direction| direction.offset())
            {
                let column = column_index as isize + x_offset;
                let row = row_index as isize + y_offset;

                if grid
                    .get_signed(column, row)
                    .is_some_and(|&test_value| test_value == val)
                {
                    let new_combined_index = column as usize + row as usize * width;
                    let _ = regions.union(combined_index, new_combined_index);
                } else {
                    perimeter += 1;
                };
            }

            perimeters[combined_index] = perimeter;
            chars[combined_index] = val;
        }
    }

    let mut sides = vec![0; width * height];
    for (row_index, row) in grid.rows().enumerate() {
        let mut upper_edge = false;
        let mut lower_edge = false;

        let mut current = grid.get(0, row_index).copied().unwrap();
        for (column_index, value) in row.into_iter().copied().enumerate() {
            let combined_index = column_index + row_index * width;
            if value != current {
                upper_edge = false;
                lower_edge = false;
                current = value;
            }

            if grid
                .get_signed(column_index as isize, row_index as isize - 1)
                .filter(|&&upper| upper == current)
                .is_some()
            {
                upper_edge = false;
            } else if !upper_edge {
                sides[combined_index] += 1;
                upper_edge = true;
            }

            if grid
                .get_signed(column_index as isize, row_index as isize + 1)
                .filter(|&&lower| lower == current)
                .is_some()
            {
                lower_edge = false;
            } else if !lower_edge {
                sides[combined_index] += 1;
                lower_edge = true;
            }
        }
    }

    for (column_index, column) in grid.columns().enumerate() {
        let mut left_edge = false;
        let mut right_edge = false;

        let mut current = grid.get(column_index, 0).copied().unwrap();
        for (row_index, value) in column.into_iter().copied().enumerate() {
            let combined_index = column_index + row_index * width;
            if value != current {
                left_edge = false;
                right_edge = false;
                current = value;
            }

            if grid
                .get_signed(column_index as isize - 1, row_index as isize)
                .filter(|&&left| left == current)
                .is_some()
            {
                left_edge = false;
            } else if !left_edge {
                sides[combined_index] += 1;
                left_edge = true;
            }

            if grid
                .get_signed(column_index as isize + 1, row_index as isize)
                .filter(|&&right| right == current)
                .is_some()
            {
                right_edge = false;
            } else if !right_edge {
                sides[combined_index] += 1;
                right_edge = true;
            }
        }
    }

    let mut map = FxHashMap::<usize, (usize, usize, usize)>::default();
    for i in 0..(width * height) {
        let perimeter = perimeters[i];
        let sides = sides[i];
        let root = regions.find(i);

        let (area_count, perimeter_count, sides_count) = map.entry(root).or_default();
        *perimeter_count += perimeter;
        *sides_count += sides;
        *area_count += 1;
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for &(area, perimeter, sides) in map.values() {
        part_1 += area * perimeter;
        part_2 += area * sides;
    }

    (part_1, part_2).into()
}
