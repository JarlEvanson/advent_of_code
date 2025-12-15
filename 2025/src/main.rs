use std::process::ExitCode;

use utils::Solution;

mod day_01;
mod day_02;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 2] = [
    Some(day_01::solve),
    Some(day_02::solve),
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
