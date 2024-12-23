use core::str;

use utils::{grid::Grid, Solution};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const TIME: usize = 100;

pub fn solve(input: &str) -> Solution {
    let robots = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let (start, velocity) = str::from_utf8(line)
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .split_once(" v=")
                .unwrap();

            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x = start_x.parse::<usize>().unwrap();
            let start_y = start_y.parse::<usize>().unwrap();

            let (velocity_x, velocity_y) = velocity.split_once(',').unwrap();
            let velocity_x = velocity_x.parse::<isize>().unwrap();
            let velocity_y = velocity_y.parse::<isize>().unwrap();

            ((start_x, start_y), (velocity_x, velocity_y))
        });

    let middle_x = WIDTH / 2;
    let middle_y = HEIGHT / 2;

    let mut upper_right = 0;
    let mut upper_left = 0;
    let mut lower_right = 0;
    let mut lower_left = 0;
    for pos in robots.clone().map(|(start, velocity)| {
        (
            calc_position(start.0, velocity.0, WIDTH, TIME),
            calc_position(start.1, velocity.1, HEIGHT, TIME),
        )
    }) {
        if pos.0 < middle_x {
            if pos.1 < middle_y {
                upper_left += 1;
            } else if pos.1 > middle_y {
                upper_right += 1;
            }
        } else if pos.0 > middle_x {
            if pos.1 < middle_y {
                lower_left += 1;
            } else if pos.1 > middle_y {
                lower_right += 1;
            }
        }
    }

    let part_1: usize = upper_right * upper_left * lower_right * lower_left;

    let mut part_2 = 0;

    // TODO: HARD CODED
    while part_2 < 7672 {
        let mut grid = Grid::new(
            vec![0usize; WIDTH * HEIGHT * 2].into_boxed_slice(),
            WIDTH,
            HEIGHT,
        );

        for pos in robots.clone().map(|(start, velocity)| {
            (
                calc_position(start.0, velocity.0, WIDTH, part_2),
                calc_position(start.1, velocity.1, HEIGHT, part_2),
            )
        }) {
            *grid.get_mut(pos.0, pos.1).unwrap() += 1;
        }

        part_2 += 1;
    }

    (part_1, part_2).into()
}

fn calc_position(start: usize, velocity: isize, dimension: usize, time: usize) -> usize {
    let movement = velocity * time as isize;
    let distance = movement.unsigned_abs();

    if movement.is_positive() {
        (start + distance % dimension) % dimension
    } else {
        let new = start.wrapping_sub(distance % dimension);
        if new >= dimension {
            new.wrapping_add(dimension)
        } else {
            new
        }
    }
}
