use std::iter;

use crate::Solution;

pub fn solve(input: &str) -> Solution {
    input
        .lines()
        .fold((0usize, 0usize), |(part_1, part_2), report| {
            let mut levels = get_levels(report);

            let mut prior_direction = Direction::Zero;
            let mut prior_level = None;

            let mut prev_direction = Direction::Zero;
            let mut prev_level = None;

            let mut current_direction = Direction::Zero;
            let Some(mut curr_level) = levels.next() else {
                return (part_1 + 1, part_2 + 1);
            };

            while let Some(next_level) = levels.next() {
                let diff = next_level as isize - curr_level as isize;

                let distance = diff.abs();
                let next_direction = Direction::from(diff);

                let distance_valid = 1 <= distance && distance <= 3;
                let direction_valid =
                    current_direction == Direction::Zero || next_direction == current_direction;

                if !distance_valid || !direction_valid {
                    let mut valid = false;

                    // Skip `next_level`.
                    valid =
                        valid || check_report(levels.clone(), current_direction, Some(curr_level));
                    // Skip `curr_level`.
                    valid = valid
                        || check_report(
                            iter::once(next_level).chain(levels.clone()),
                            prev_direction,
                            prev_level,
                        );

                    // Skip `prev_level`.
                    valid = valid
                        || check_report(
                            iter::once(curr_level)
                                .chain(iter::once(next_level))
                                .chain(levels.clone()),
                            prior_direction,
                            prior_level,
                        );

                    return (part_1, part_2 + valid as usize);
                }

                prior_level = prev_level;
                prior_direction = prev_direction;

                prev_level = Some(curr_level);
                prev_direction = current_direction;

                curr_level = next_level;
                current_direction = next_direction;
            }

            (part_1 + 1, part_2 + 1)
        })
        .into()
}

pub fn check_report(
    mut levels: impl Iterator<Item = usize> + Clone,
    mut current_direction: Direction,
    current_level: Option<usize>,
) -> bool {
    let Some(mut current_level) = current_level.or_else(|| levels.next()) else {
        return true;
    };

    for next_level in levels {
        let diff = next_level as isize - current_level as isize;

        let distance = diff.abs();
        if !(1 <= distance && distance <= 3) {
            return false;
        }

        let next_direction = Direction::from(diff);
        if current_direction != Direction::Zero && next_direction != current_direction {
            return false;
        }

        current_level = next_level;
        current_direction = next_direction;
    }

    true
}

#[inline]
pub fn get_levels(report: &str) -> impl Iterator<Item = usize> + Clone + use<'_> {
    report
        .split_ascii_whitespace()
        .map(|level| level.parse::<usize>().unwrap())
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Increasing,
    Decreasing,
    Zero,
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            value if value > 0 => Self::Increasing,
            0 => Self::Zero,
            _ => Self::Decreasing,
        }
    }
}
