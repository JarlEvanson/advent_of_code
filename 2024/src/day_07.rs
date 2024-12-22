use utils::Solution;

pub fn solve(input: &str) -> Solution {
    input
        .lines()
        .fold((0usize, 0usize), |(part_1, part_2), line| {
            let (target, remainder) = line.split_once(": ").unwrap();

            let target = target.parse::<usize>().unwrap();

            let numbers = remainder
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut part_1_valid = false;
            let mut part_2_valid = false;

            step(
                target,
                numbers[0],
                &numbers[1..],
                false,
                &mut part_1_valid,
                &mut part_2_valid,
            );

            (
                part_1 + part_1_valid as usize * target,
                part_2 + part_2_valid as usize * target,
            )
        })
        .into()
}

fn step(
    target: usize,
    left: usize,
    remaining: &[usize],
    used_concat: bool,
    part_1: &mut bool,
    part_2: &mut bool,
) {
    if remaining.len() == 0 || left > target {
        let result = left == target;

        *part_2 = *part_2 || result;
        if !used_concat {
            *part_1 = *part_1 || result;
        }

        return;
    }

    step(
        target,
        left * remaining[0],
        &remaining[1..],
        used_concat,
        part_1,
        part_2,
    );
    if *part_1 && *part_2 {
        return;
    }

    step(
        target,
        left + remaining[0],
        &remaining[1..],
        used_concat,
        part_1,
        part_2,
    );
    if *part_1 && *part_2 {
        return;
    }

    step(
        target,
        concat(left, remaining[0]),
        &remaining[1..],
        true,
        part_1,
        part_2,
    )
}

fn concat(left: usize, val: usize) -> usize {
    left * (10usize.pow(format!("{val}").len() as u32)) + val
}
