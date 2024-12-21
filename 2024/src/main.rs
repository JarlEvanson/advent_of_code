use std::process::ExitCode;

use utils::Solution;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 25] = [
    Some(day_01::solve),
    Some(day_02::solve),
    Some(day_03::solve),
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
