use utils::Solution;

const STARTING_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub fn solve(input: &str) -> Solution {
    let lines = input.lines();
    let actions = lines.map(|line| line.split_at(1)).map(|(action, count)| {
        let is_right = action.starts_with('R');
        let count = count.parse::<i32>().unwrap();
        if is_right {
            count
        } else {
            -count
        }
    });

    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut position = STARTING_POSITION;
    for action in actions {
        let unrounded_position = position + action;

        part_1 += (unrounded_position.rem_euclid(100) == 0) as usize;
        part_2 += unrounded_position.div_euclid(100).unsigned_abs() as usize;
        part_2 = part_2.strict_add_signed(
            action.is_negative() as isize
                * ((unrounded_position.rem_euclid(100) == 0) as isize - (position == 0) as isize),
        );

        position = unrounded_position.rem_euclid(DIAL_SIZE);
    }

    (part_1, part_2).into()
}
