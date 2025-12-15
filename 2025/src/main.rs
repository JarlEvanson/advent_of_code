use std::process::ExitCode;

use utils::Solution;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 4] = [
    Some(day_01::solve),
    Some(day_02::solve),
    Some(day_03::solve),
    Some(day_04::solve),
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
