use utils::{direction::PrincipalDirection, grid::Grid, hash::FxHashSet, Solution};

pub fn solve(input: &str) -> Solution {
    let mut height = 0;
    let mut start_pos = (0, 0);
    let mut chars = Vec::new();
    for (line_index, line) in input.lines().enumerate() {
        for (column_index, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    chars.push(State::Beam);
                    start_pos = (column_index, line_index)
                }
                '.' => chars.push(State::Empty),
                '^' => chars.push(State::Splitter),
                _ => unreachable!(),
            }
        }

        height += 1;
    }

    let width = chars.len() / height;
    let grid = Grid::new(chars.into_boxed_slice(), width, height);

    let mut part_1 = 0usize;
    let mut part_1_grid = grid;
    for row_index in 0..height {
        for column_index in 0..width {
            if &State::Beam != part_1_grid.get(column_index, row_index).unwrap() {
                // State::Empty or State::Splitter
                continue;
            }

            // State::Beam
            match part_1_grid.get_dir_mut(column_index, row_index, PrincipalDirection::South, 1) {
                Some(State::Splitter) => {
                    part_1 += 1;
                    for dir in [PrincipalDirection::SouthWest, PrincipalDirection::SouthEast] {
                        let Some(state) = part_1_grid.get_dir_mut(column_index, row_index, dir, 1)
                        else {
                            continue;
                        };

                        if state == &State::Empty {
                            *state = State::Beam;
                        }
                    }
                }
                Some(state @ &mut State::Empty) => *state = State::Beam,
                _ => {}
            }
        }
    }

    (part_1).into()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Beam,
    Splitter,
}
