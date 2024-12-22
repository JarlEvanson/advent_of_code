use utils::{hash::FxHashMap, Solution};

pub fn solve(input: &str) -> Solution {
    let stones = input
        .split_ascii_whitespace()
        .map(|number| number.parse::<usize>().unwrap());

    let mut old_counts = FxHashMap::default();
    let mut counts = FxHashMap::default();

    for stone in stones {
        *counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..25 {
        std::mem::swap(&mut old_counts, &mut counts);

        blink(&old_counts, &mut counts);
    }

    let part_1 = counts.iter().map(|(&_, &count)| count).sum::<usize>();

    for _ in 25..75 {
        std::mem::swap(&mut old_counts, &mut counts);

        blink(&old_counts, &mut counts);
    }

    let part_2 = counts.iter().map(|(&_, &count)| count).sum::<usize>();

    (part_1, part_2).into()
}

pub fn blink(old: &FxHashMap<usize, usize>, new: &mut FxHashMap<usize, usize>) {
    new.clear();

    for (&stone, &count) in old.iter() {
        if stone == 0 {
            *new.entry(1).or_insert(0) += count;
        } else if get_digit_count(stone) % 2 == 0 {
            let pow = get_digit_count(stone) / 2;
            let divisor = 10usize.pow(pow);

            *new.entry(stone / divisor).or_insert(0) += count;
            *new.entry(stone % divisor).or_insert(0) += count;
        } else {
            *new.entry(stone * 2024).or_insert(0) += count;
        }
    }
}

pub fn get_digit_count(mut number: usize) -> u32 {
    let mut digits = 0;
    loop {
        number /= 10;
        digits += 1;

        if number == 0 {
            break;
        }
    }

    digits
}
