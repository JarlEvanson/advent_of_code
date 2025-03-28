use core::str;

use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let machines = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .filter(|line| line.len() != 0)
        .array_chunks()
        .map(|[a, b, prize]| {
            let prize = str::from_utf8(prize)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let prize_x = prize.0.split_once('=').unwrap().1.parse::<usize>().unwrap();
            let prize_y = prize.1.split_once('=').unwrap().1.parse::<usize>().unwrap();

            let a = str::from_utf8(a)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let a_x = a.0.split_once('+').unwrap().1.parse::<usize>().unwrap();
            let a_y = a.1.split_once('+').unwrap().1.parse::<usize>().unwrap();

            let b = str::from_utf8(b)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let b_x = b.0.split_once('+').unwrap().1.parse::<usize>().unwrap();
            let b_y = b.1.split_once('+').unwrap().1.parse::<usize>().unwrap();

            ((a_x, a_y), (b_x, b_y), (prize_x, prize_y))
        });

    let mut part_1 = 0;
    let mut part_2 = 0;
    for (a, b, prize) in machines.clone() {
        let a = (a.0 as i64, a.1 as i64);
        let b = (b.0 as i64, b.1 as i64);
        let prize = (prize.0 as i64, prize.1 as i64);

        if let Some(solution) = solve_claw(a, b, prize) {
            part_1 += solution.0 * 3 + solution.1;
        }

        let prize = (
            prize.0 as i64 + 10000000000000,
            prize.1 as i64 + 10000000000000,
        );

        if let Some(solution) = solve_claw(a, b, prize) {
            part_2 += solution.0 * 3 + solution.1;
        }
    }

    (part_1, part_2).into()
}

pub fn solve_claw(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> Option<(usize, usize)> {
    let a_count = (prize.0 * b.1 - prize.1 * b.0) / (a.0 * b.1 - a.1 * b.0);
    let b_count = (prize.0 * a.1 - prize.1 * a.0) / (b.0 * a.1 - b.1 * a.0);

    if a_count * a.0 + b_count * b.0 == prize.0
        && a_count * a.1 + b_count * b.1 == prize.1
        && !a_count.is_negative()
        && !b_count.is_negative()
    {
        Some((a_count as usize, b_count as usize))
    } else {
        None
    }
}
