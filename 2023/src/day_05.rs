use std::ops::Range;

use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let mut lines = input.lines();

    let seeds = lines
        .clone()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|str| str.parse::<u64>().unwrap())
        .map(|seed| seed..seed + 1)
        .collect::<Vec<_>>();

    let seed_ranges = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|str| str.parse::<u64>().unwrap())
        .array_chunks::<2>()
        .map(|[a, b]| (a..(a + b)))
        .collect::<Vec<_>>();

    (
        solve_part(lines.clone(), seeds),
        solve_part(lines, seed_ranges),
    )
        .into()
}

fn solve_part<'input>(
    lines: impl Iterator<Item = &'input str>,
    mut seed_ranges: Vec<Range<u64>>,
) -> u64 {
    let mut lines = lines.peekable();

    let mut new_seeds = Vec::new();

    while lines.next() == Some("") {
        lines.next();

        let mut maps = Vec::new();

        while lines.peek() != Some(&"") && lines.peek().is_some() {
            let mut line = lines.next().unwrap().split_ascii_whitespace();

            let dest = line.next().unwrap().parse::<u64>().unwrap();
            let src = line.next().unwrap().parse::<u64>().unwrap();
            let len = line.next().unwrap().parse::<u64>().unwrap();

            maps.push(((src..src + len), dest));
        }

        'seed_processing: while !seed_ranges.is_empty() {
            let seed = seed_ranges.swap_remove(0);

            'map_ranges: for (range, dest_offset) in maps.iter() {
                if range.start >= seed.end || seed.start >= range.end {
                    continue 'map_ranges;
                }

                let output_start = u64::max(seed.start, range.start);
                let output_end = u64::min(seed.end, range.end);

                let mapped_start = (output_start - range.start) + dest_offset;
                let mapped_end = (output_end - range.start) + dest_offset;

                new_seeds.push(mapped_start..mapped_end);

                if output_start != seed.start {
                    let low_start = seed.start;
                    let low_end = output_start;
                    seed_ranges.push(low_start..low_end);
                }
                if output_end != seed.end {
                    let high_start = output_end;
                    let high_end = seed.end;
                    seed_ranges.push(high_start..high_end);
                }
                continue 'seed_processing;
            }

            new_seeds.push(seed);
        }

        seed_ranges = new_seeds;
        new_seeds = Vec::new();
    }

    seed_ranges
        .iter()
        .map(|seed_range| seed_range.start)
        .min()
        .unwrap()
}
