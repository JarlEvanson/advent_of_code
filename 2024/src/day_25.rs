use utils::{grid::Grid, Solution};

pub fn solve(input: &str) -> Solution {
    let schematics = input.split("\n\n").map(|schematic| {
        let data = schematic
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Grid::new(data, 5, 7)
    });

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in schematics {
        let mut vec = Vec::new();

        if schematic.get(0, 0).copied().unwrap() == '#' {
            // Lock
            for column in schematic.columns() {
                vec.push(column.into_iter().position(|&c| c == '.').unwrap());
            }

            locks.push(vec);
        } else {
            for column in schematic.columns() {
                vec.push(7 - column.into_iter().position(|&c| c == '#').unwrap());
            }

            keys.push(vec);
        }
    }

    let mut part_1: u64 = 0;
    for lock in locks.iter() {
        'next: for key in keys.iter() {
            for (&lock, &key) in lock.iter().zip(key.iter()) {
                if lock + key > 7 {
                    continue 'next;
                }
            }

            part_1 += 1;
        }
    }

    part_1.into()
}
