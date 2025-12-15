use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let mut fresh = Vec::new();
    let mut available = Vec::new();

    let mut parsing_fresh = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_fresh = false;
            continue;
        }

        if parsing_fresh {
            let (lower, upper) = line.trim().split_once('-').unwrap();
            let lower = lower.parse::<u64>().unwrap();
            let upper = upper.parse::<u64>().unwrap();

            fresh.push(lower..=upper);
        } else {
            available.push(line.trim().parse::<u64>().unwrap());
        }
    }

    fresh.sort_by_key(|range| *range.start());

    let mut index = 0;
    while index < fresh.len() {
        let mut range = fresh[index].clone();

        let mut merge_index = index + 1;
        while merge_index < fresh.len() {
            let merge_range = fresh[merge_index].clone();

            if merge_range.start() <= range.end() {
                let start = *range.start();
                let end = *range.end().max(merge_range.end());

                range = start..=end;
                merge_index += 1;
            } else {
                break;
            }
        }

        fresh.drain(index..(merge_index - 1));

        fresh[index] = range;
        index += 1;
    }

    let mut part_1 = 0usize;
    for &id in available.iter() {
        let result = fresh.binary_search_by_key(&id, |range| *range.end());
        let index = match result {
            Ok(index) => index,
            Err(index) => index,
        };

        let Some(range) = fresh.get(index) else {
            continue;
        };

        part_1 += range.contains(&id) as usize;
    }

    let part_2 = fresh
        .into_iter()
        //.inspect(|range| println!("{range:?}"))
        .map(|range| *range.end() - *range.start() + 1)
        .sum::<u64>();

    (part_1, part_2 as usize).into()
}
