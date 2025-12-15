use utils::{hash::FxHashSet, Solution};

pub fn solve(input: &str) -> Solution {
    let ranges = input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(lower, upper)| lower.parse::<u64>().unwrap()..=upper.parse::<u64>().unwrap());

    let mut part_1 = 0;
    let mut part_2 = FxHashSet::default();
    for range in ranges {
        let start_digits = range.start().checked_ilog10().unwrap_or(0) + 1;
        let end_digits = range.end().checked_ilog10().unwrap_or(0) + 1;
        for digits in start_digits..=end_digits {
            if digits % 2 == 1 {
                // Only numbers with the number of digits divisible by two can be valid.
                continue;
            }

            let mult = 10u64.pow(digits / 2);

            let mut start = mult / 10;
            while start * mult + start < *range.start() {
                start += 1;
            }

            let mut end = mult - 1;
            while end * mult + end > *range.end() {
                end -= 1;
            }

            let mut index = start;
            while index <= end {
                part_1 += index * mult + index;
                index += 1;
            }
        }

        let mut num = 1;
        let max_digits = range.end().isqrt().checked_ilog10().unwrap_or(0) + 1;
        let max_num = 10u64.pow(max_digits).saturating_sub(1);
        while num <= max_num {
            let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
            let mult = 10u64.pow(num_digits);
            let start_reps = start_digits / num_digits;
            let end_reps = end_digits / num_digits;

            for rep_count in start_reps.max(2)..=end_reps.max(2) {
                let mut number = num;
                let mut counter = 1;
                while counter < rep_count {
                    number = number * mult + num;
                    counter += 1;
                }

                if number < *range.start() || *range.end() < number {
                    continue;
                }

                part_2.insert(number);
            }

            num += 1;
        }
    }

    (part_1, part_2.iter().sum::<u64>() as usize).into()
}
