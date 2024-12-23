#![feature(iter_map_windows)]

use std::process::ExitCode;

use utils::Solution;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_21;
pub mod day_22;

pub const SOLUTIONS: [Option<fn(&str) -> Solution>; 25] = [
    Some(day_01::solve),
    Some(day_02::solve),
    Some(day_03::solve),
    Some(day_04::solve),
    Some(day_05::solve),
    Some(day_06::solve),
    Some(day_07::solve),
    Some(day_08::solve),
    Some(day_09::solve),
    Some(day_10::solve),
    Some(day_11::solve),
    None,
    None,
    None,
    None,
    Some(day_16::solve),
    Some(day_17::solve),
    Some(day_18::solve),
    Some(day_19::solve),
    None,
    Some(day_21::solve),
    Some(day_22::solve),
    None,
    None,
    None,
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
