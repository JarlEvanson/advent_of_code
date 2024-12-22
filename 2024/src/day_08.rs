use utils::{
    grid::Grid,
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let mut frequency_antennas = FxHashMap::default();
    for row in 0..grid.height() {
        for column in 0..grid.width() {
            let value = *grid.get(column, row).unwrap();

            if value != b'.' {
                let Some(antennas) = frequency_antennas.get_mut(&value) else {
                    frequency_antennas.insert(value, vec![(column, row)]);
                    continue;
                };

                antennas.push((column, row));
            }
        }
    }

    let mut part_1_antinodes = FxHashSet::default();
    let mut part_2_antinodes = FxHashSet::default();
    for antennas in frequency_antennas.values() {
        for (index, antenna_a) in antennas.iter().enumerate() {
            for antenna_b in antennas
                .get(index + 1..)
                .iter()
                .flat_map(|antennas| antennas.iter())
            {
                part_2_antinodes.insert((antenna_a.0 as isize, antenna_a.1 as isize));
                part_2_antinodes.insert((antenna_b.0 as isize, antenna_b.1 as isize));

                let x_offset = antenna_a.0 as isize - antenna_b.0 as isize;
                let y_offset = antenna_a.1 as isize - antenna_b.1 as isize;

                let mut antinode = (
                    antenna_a.0 as isize + x_offset,
                    antenna_a.1 as isize + y_offset,
                );
                if grid.get_signed(antinode.0, antinode.1).is_some() {
                    part_1_antinodes.insert(antinode);
                }
                while grid.get_signed(antinode.0, antinode.1).is_some() {
                    part_2_antinodes.insert(antinode);
                    antinode = (antinode.0 + x_offset, antinode.1 + y_offset);
                }

                antinode = (
                    antenna_b.0 as isize - x_offset,
                    antenna_b.1 as isize - y_offset,
                );
                if grid.get_signed(antinode.0, antinode.1).is_some() {
                    part_1_antinodes.insert(antinode);
                }
                while grid.get_signed(antinode.0, antinode.1).is_some() {
                    part_2_antinodes.insert(antinode);
                    antinode = (antinode.0 - x_offset, antinode.1 - y_offset);
                }
            }
        }
    }

    (part_1_antinodes.len(), part_2_antinodes.len()).into()
}
