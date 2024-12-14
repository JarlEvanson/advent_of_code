#![feature(linked_list_cursors)]
#![feature(iter_array_chunks)]

use std::process::ExitCode;

use utils::unimplemented;

pub mod day_01;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;

pub const SOLUTIONS: [fn(&str) -> (usize, usize); 25] = [
    day_01::solve,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    day_08::solve,
    day_09::solve,
    day_10::solve,
    day_11::solve,
    day_12::solve,
    day_13::solve,
    day_14::solve,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
];

fn main() -> ExitCode {
    utils::run(SOLUTIONS)
}
