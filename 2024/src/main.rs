use std::process::ExitCode;

use utils::unimplemented;

pub mod day_01;
pub mod day_08;
pub mod day_09;

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
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
    unimplemented,
];

fn main() -> ExitCode {
    utils::run(SOLUTIONS)
}
