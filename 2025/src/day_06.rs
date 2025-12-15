use std::fmt::{self, Display};

use utils::{grid::Grid, Solution};

pub fn solve(input: &str) -> Solution {
    let mut ops = Vec::new();
    let mut part_1_numbers: Vec<Vec<u64>> = Vec::new();
    let mut chars = Vec::new();
    let mut height = 0;

    for (line_index, line) in input.lines().enumerate() {
        let mut whitespace = true;
        let mut problem_index = usize::MAX;
        for c in line.chars() {
            chars.push(c);
            match c {
                c if c.is_whitespace() => whitespace = true,
                c if c.is_numeric() => {
                    if whitespace {
                        problem_index = problem_index.wrapping_add(1);
                        whitespace = false;
                    }

                    if part_1_numbers.get(problem_index).is_none() {
                        part_1_numbers.push(vec![]);
                    }

                    if part_1_numbers[problem_index].get(line_index).is_none() {
                        part_1_numbers[problem_index].push(0u64);
                    }

                    let value = &mut part_1_numbers[problem_index][line_index];
                    *value = *value * 10 + c as u64 - b'0' as u64;
                }
                '+' => ops.push(Op::Add),
                '*' => ops.push(Op::Multiply),
                _ => unreachable!(),
            }
        }

        height += 1;
    }

    let width = chars.len() / height;
    height -= 1;
    chars.truncate(chars.len() - width);

    let mut part_1 = 0usize;
    for (op, numbers) in ops.iter().zip(part_1_numbers.iter()) {
        let mut result = numbers[0];

        for number in numbers.iter().skip(1).copied() {
            match op {
                Op::Multiply => result *= number,
                Op::Add => result += number,
            }
        }

        part_1 += result as usize;
    }

    let grid = Grid::new(chars.into_boxed_slice(), width, height);

    let mut part_2_numbers = vec![const { Vec::new() }; part_1_numbers.len()];
    let mut problem_index = 0;
    for column in grid.columns() {
        let mut number = 0;
        for &c in column.into_iter().filter(|c: &&char| c.is_numeric()) {
            number = number * 10 + c as u64 - b'0' as u64;
        }

        if number == 0 {
            problem_index += 1;
        } else {
            part_2_numbers[problem_index].push(number);
        }
    }

    let mut part_2 = 0usize;
    for (op, numbers) in ops.iter().zip(part_2_numbers.iter()) {
        let mut result = numbers[0];

        for number in numbers.iter().skip(1).copied() {
            match op {
                Op::Multiply => result *= number,
                Op::Add => result += number,
            }
        }

        part_2 += result as usize;
    }

    (part_1, part_2).into()
}

#[derive(Clone, Copy)]
enum Op {
    Multiply,
    Add,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Multiply => "*".fmt(f),
            Op::Add => "+".fmt(f),
        }
    }
}
