use std::process::ExitCode;

use utils::unimplemented;

pub mod day_01;

pub const SOLUTIONS: [fn(&str) -> (usize, usize); 25] = [
    day_01::solve,
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
