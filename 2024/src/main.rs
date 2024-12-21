use std::process::ExitCode;

use utils::Solution;

mod day_01;
mod day_02;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 25] = [
    Some(day_01::solve),
    Some(day_02::solve),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
