use std::process::ExitCode;

use utils::Solution;

mod day_01;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 1] = [
    Some(day_01::solve),
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
