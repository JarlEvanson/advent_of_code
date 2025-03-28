use utils::{direction::PrincipalDirection, grid::Grid, Solution};

pub fn solve(input: &str) -> Solution {
    let mut part_1 = 0;

    let grid = Grid::new(
        input
            .lines()
            .flat_map(str::chars)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );

    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut digits = line
            .match_indices(|ch| char::is_ascii_digit(&ch))
            .peekable();

        while let Some((start, _)) = digits.next() {
            let mut curr_index = start;

            while let Some((next_index, _)) = digits.peek() {
                if *next_index == curr_index + 1 {
                    curr_index = *next_index;
                    digits.next();
                } else {
                    break;
                }
            }

            let num = line[start..=curr_index].parse::<usize>().unwrap();

            numbers.push(Number {
                val: num,
                x: start,
                y,
                length: curr_index - start + 1,
            });

            'test_loop: for x in start..=curr_index {
                for (x_offset, y_offset) in
                    PrincipalDirection::ALL.map(|direction| direction.offset())
                {
                    let test_x = x.wrapping_add_signed(x_offset);
                    let test_y = y.wrapping_add_signed(y_offset);

                    if let Some(test) = grid.get(test_x, test_y).copied() {
                        if !(test.is_ascii_alphanumeric() || test == '.') {
                            part_1 += num;
                            break 'test_loop;
                        }
                    }
                }
            }
        }
    }

    let mut adjacent = Vec::with_capacity(8);

    let potential_gears = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.match_indices('*').map(move |(x, _)| (x, y)))
        .filter_map(|(x, y)| {
            if parts_around_gear(&grid, (x, y)) != 2 {
                return None;
            }

            adjacent.clear();
            for (x_offset, y_offset) in PrincipalDirection::ALL.map(|direction| direction.offset())
            {
                let test_x = x.wrapping_add_signed(x_offset);
                let test_y = y.wrapping_add_signed(y_offset);

                if grid
                    .get(test_x, test_y)
                    .copied()
                    .is_some_and(|test| test.is_ascii_digit())
                {
                    if let Some(number) = numbers.iter().find(|num| num.contains(test_x, test_y)) {
                        adjacent.push(*number);
                    }
                }
            }

            adjacent.dedup_by(|a, b| a == b);
            Some((adjacent[0], adjacent[1]))
        })
        .map(|(a, b)| a.val * b.val)
        .sum::<usize>();

    (part_1, potential_gears).into()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Number {
    val: usize,
    x: usize,
    y: usize,
    length: usize,
}

impl Number {
    fn contains(&self, x: usize, y: usize) -> bool {
        if y != self.y {
            return false;
        }

        (self.x..=(self.x + self.length)).contains(&x)
    }
}

fn parts_around_gear(grid: &Grid<char>, position: (usize, usize)) -> u8 {
    let position: (isize, isize) = (position.0 as isize, position.1 as isize);

    let mut count = 0;
    if grid
        .get_signed(position.0, position.1 - 1)
        .is_some_and(|&c| c.is_ascii_alphanumeric())
    {
        count += 1;
    } else {
        count += grid
            .get_signed(position.0 - 1, position.1 - 1)
            .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;
        count += grid
            .get_signed(position.0 + 1, position.1 - 1)
            .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;
    }

    count += grid
        .get_signed(position.0 - 1, position.1)
        .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;
    count += grid
        .get_signed(position.0 + 1, position.1)
        .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;

    if grid
        .get_signed(position.0, position.1 + 1)
        .is_some_and(|&c| c.is_ascii_alphanumeric())
    {
        count += 1;
    } else {
        count += grid
            .get_signed(position.0 - 1, position.1 + 1)
            .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;
        count += grid
            .get_signed(position.0 + 1, position.1 + 1)
            .is_some_and(|&c| c.is_ascii_alphanumeric()) as u8;
    }

    count
}
