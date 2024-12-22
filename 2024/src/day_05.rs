use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();

            (
                before.parse::<usize>().unwrap(),
                after.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut part_1 = 0;
    let mut part_2 = 0usize;

    for line in updates.lines() {
        let mut update = line
            .split(',')
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        'part_1: {
            for (before, after) in rules.iter().copied() {
                let mut before_pos = None;
                let mut after_pos = None;
                for (index, value) in update.iter().copied().enumerate() {
                    if value == before && before_pos.is_none() {
                        before_pos = Some(index);

                        if after_pos.is_some() {
                            break;
                        }
                    }

                    if value == after && after_pos.is_none() {
                        after_pos = Some(index);

                        if before_pos.is_some() {
                            break;
                        }
                    }
                }

                let Some((before_pos, after_pos)) = before_pos.zip(after_pos) else {
                    continue;
                };

                if before_pos > after_pos {
                    break 'part_1;
                }
            }

            part_1 += update[update.len() / 2];
        }

        let mut correct = true;
        loop {
            let mut changed = false;

            for (before, after) in rules.iter().copied() {
                let mut before_pos = None;
                let mut after_pos = None;
                for (index, value) in update.iter().copied().enumerate() {
                    if value == before && before_pos.is_none() {
                        before_pos = Some(index);

                        if after_pos.is_some() {
                            break;
                        }
                    }

                    if value == after && after_pos.is_none() {
                        after_pos = Some(index);

                        if before_pos.is_some() {
                            break;
                        }
                    }
                }

                let Some((before_pos, after_pos)) = before_pos.zip(after_pos) else {
                    continue;
                };

                if before_pos > after_pos {
                    update.swap(before_pos, after_pos);
                    correct = false;
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }

        if !correct {
            part_2 += update[update.len() / 2];
        }
    }

    (part_1, part_2).into()
}
