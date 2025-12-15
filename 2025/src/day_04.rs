use utils::{direction::PrincipalDirection, grid::Grid, hash::FxHashSet, Solution};

pub fn solve(input: &str) -> Solution {
    let grid = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c == '@'))
        .collect::<Vec<_>>();
    let width = input.lines().next().unwrap().chars().count();
    let height = grid.len() / width;

    let mut grid = Grid::new(grid.into_boxed_slice(), width, height);
    let mut adjacency_grid = Grid::new(
        vec![0u8; grid.width() * grid.height()].into_boxed_slice(),
        grid.width(),
        grid.height(),
    );

    for (x, y, &occupied) in grid.iter() {
        if !occupied {
            continue;
        }

        for (index, direction) in PrincipalDirection::ALL.into_iter().enumerate() {
            let Some(&occupied) = grid.get_dir(x, y, direction, 1) else {
                continue;
            };

            if occupied {
                *adjacency_grid.get_mut(x, y).unwrap() |= 1 << index;
            }
        }
    }

    let mut removable = FxHashSet::default();
    let part_1 = grid
        .iter()
        .map(|(x, y, &val)| (x, y, val))
        .zip(
            adjacency_grid
                .iter()
                .map(|(_, _, &val)| val.count_ones() < 4),
        )
        .map(|((x, y, occupied), valid)| (x, y, occupied && valid))
        .inspect(|&(x, y, is_removable)| {
            if is_removable {
                removable.insert((x, y));
            }
        })
        .filter(|&(_, _, is_removable)| is_removable)
        .count();

    let mut part_2 = 0;
    while !removable.is_empty() {
        let (x, y) = removable
            .take(&removable.iter().next().copied().unwrap())
            .unwrap();

        *grid.get_mut(x, y).unwrap() = false;
        for (index, direction) in PrincipalDirection::ALL.into_iter().enumerate() {
            let Some(adjacency) = adjacency_grid.get_dir_mut(x, y, direction, 1) else {
                continue;
            };

            let index = index.wrapping_add(4) % 8;
            *adjacency &= !(1 << index);

            if adjacency.count_ones() >= 4 {
                continue;
            }

            if grid.get_dir(x, y, direction, 1).is_some_and(|&val| val) {
                let x = x.checked_add_signed(direction.offset().0).unwrap();
                let y = y.checked_add_signed(direction.offset().1).unwrap();

                removable.insert((x, y));
            }
        }

        part_2 += 1usize;
    }

    (part_1, part_2).into()
}
