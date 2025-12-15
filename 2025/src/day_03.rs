use std::cmp::Ordering;

use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let lines = input.lines();
    let banks = lines
        .map(|line| {
            line.chars()
                .map(|c| (c as u32 - '0' as u32) as u8)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for bank in banks.iter() {
        part_1 += find_largest_joltage::<2>(bank);
        part_2 += find_largest_joltage::<12>(bank);
    }

    (part_1, part_2).into()
}

fn find_largest_joltage<const N: usize>(joltage: &[u8]) -> usize {
    let mut skip = 0;
    let mut number = 0;
    for index in 0..N {
        let back_skip_count = N - (index + 1);
        let (index, &num) = joltage[skip..]
            .iter()
            .rev()
            .skip(back_skip_count)
            .rev()
            .enumerate()
            .max_by(|a, b| match a.1.cmp(b.1) {
                Ordering::Equal => match a.0.cmp(&b.0) {
                    Ordering::Less => Ordering::Greater,
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Greater => Ordering::Less,
                },
                ord => ord,
            })
            .unwrap();

        skip += index + 1;
        number = number * 10 + usize::from(num);
    }

    number
}
