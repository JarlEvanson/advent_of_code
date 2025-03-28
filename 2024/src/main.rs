#![feature(iter_array_chunks)]
#![feature(iter_map_windows)]

use std::process::ExitCode;

use utils::Solution;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

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
    Some(day_12::solve),
    Some(day_13::solve),
    Some(day_14::solve),
    Some(day_15::solve),
    Some(day_16::solve),
    Some(day_17::solve),
    Some(day_18::solve),
    Some(day_19::solve),
    Some(day_20::solve),
    Some(day_21::solve),
    Some(day_22::solve),
    Some(day_23::solve),
    Some(day_24::solve),
    Some(day_25::solve),
];

fn main() -> ExitCode {
    utils::cli::run(SOLUTIONS)
}
