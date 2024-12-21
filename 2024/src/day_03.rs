use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let mut cursor = 0;

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut part_2_enabled = true;
    while let Some(remaining) = input.get(cursor..) {
        'action_block: {
            if remaining.starts_with("do()") {
                part_2_enabled = true;
            } else if remaining.starts_with("don't()") {
                part_2_enabled = false;
            } else if remaining.starts_with("mul(") {
                let Some((numbers, _)) = remaining
                    .get(4..)
                    .map(|potential| potential.split_once(')'))
                    .flatten()
                else {
                    break 'action_block;
                };

                let Some((num_1, num_2)) = numbers.split_once(',') else {
                    break 'action_block;
                };

                let Some(num_1) = num_1.parse::<usize>().ok() else {
                    break 'action_block;
                };

                let Some(num_2) = num_2.parse::<usize>().ok() else {
                    break 'action_block;
                };

                part_1 += num_1 * num_2;
                if part_2_enabled {
                    part_2 += num_1 * num_2;
                }
            }
        }

        cursor += remaining
            .get(1..)
            .map(|remaining| remaining.find(['m', 'd']).map(|offset| offset + 1))
            .flatten()
            .unwrap_or(input.len() - remaining.len());
    }

    (part_1, part_2).into()
}
