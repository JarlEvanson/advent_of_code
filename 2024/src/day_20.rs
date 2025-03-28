use utils::{direction::Direction, grid::Grid, hash::FxHashMap, Solution};

pub fn solve(input: &str) -> Solution {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let mut data = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            let walkable = match c {
                '#' => false,
                '.' => true,
                'S' => {
                    start_pos = (column, row);
                    true
                }
                'E' => {
                    end_pos = (column, row);
                    true
                }
                _ => unreachable!(),
            };
            data.push(walkable);
        }
    }

    let data = data.into_boxed_slice();

    let width = input
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let end_direction = 'direction_find: {
        for direction in [Direction::North, Direction::South, Direction::West] {
            if grid
                .get_signed(
                    end_pos.0 as isize + direction.offset().0,
                    end_pos.1 as isize + direction.offset().1,
                )
                .copied()
                .unwrap()
            {
                break 'direction_find direction;
            }
        }

        Direction::East
    };

    let mut part_1: u64 = 0;
    let mut part_2: u64 = 0;

    let mut pos = end_pos;
    let mut dir = end_direction;
    let mut legal_time = 0;
    let mut cache = FxHashMap::default();
    'backwalk: loop {
        cache.insert(pos, legal_time);
        legal_time += 1;

        if legal_time >= 100 {
            let min_x = pos.0.saturating_sub(21);
            let max_x = (pos.0 + 21).min(grid.width());

            let min_y = pos.1.saturating_sub(21);
            let max_y = (pos.1 + 21).min(grid.height());

            for test_y in min_y..max_y {
                for test_x in min_x..max_x {
                    let manhattan_distance = test_x.abs_diff(pos.0) + test_y.abs_diff(pos.1);
                    if manhattan_distance > 20 {
                        continue;
                    }

                    if let Some(&remaining) = cache.get(&(test_x, test_y)) {
                        if legal_time - (remaining + manhattan_distance) < 100 {
                            continue;
                        }

                        if manhattan_distance == 2 {
                            part_1 += 1;
                        }
                        part_2 += 1;
                    }
                }
            }
        }

        if pos == start_pos {
            break;
        }

        let test_directions = [dir, dir.clockwise(), dir.counterclockwise()];
        for test_dir in test_directions {
            let test_pos = (
                pos.0 as isize + test_dir.offset().0,
                pos.1 as isize + test_dir.offset().1,
            );

            if grid
                .get_signed(test_pos.0, test_pos.1)
                .is_some_and(|&walkable| walkable)
            {
                dir = test_dir;
                pos = (test_pos.0 as usize, test_pos.1 as usize);
                continue 'backwalk;
            }
        }

        break;
    }

    (part_1, part_2).into()
}
