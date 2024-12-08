use std::{
    env::args,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{self, Duration},
};

use criterion::Criterion;

pub mod direction;
pub mod grid;
pub mod hash;

pub fn run(solutions: [fn(&str) -> (usize, usize); 25]) -> ExitCode {
    let (action, input_folder, days) = match get_action() {
        Ok((action, input_folder, days)) => (action, input_folder, days),
        Err(error) => {
            print!("{error}");
            return ExitCode::FAILURE;
        }
    };

    if !input_folder.is_dir() {
        println!("Invalid inputs folder");
        return ExitCode::FAILURE;
    }

    match action {
        Action::Solve => {
            let mut total_time = Duration::new(0, 0);
            for day in days {
                let Some(input) = acquire_input(input_folder.as_path(), day) else {
                    println!("Skipping day {day:02}");
                    continue;
                };

                let (duration, (part_1, part_2)) = solve(&input, solutions[(day - 1) as usize]);
                total_time += duration;

                println!("Day {day:02}:");
                println!("\tPart 1: {part_1}");
                println!("\tPart 2: {part_2}");
                println!("\tTime: {duration:?}");
            }

            println!("Total Processing Time: {total_time:?}");
        }
        Action::Benchmark => {
            let mut criterion = Criterion::default();

            for day in days {
                let Some(input) = acquire_input(input_folder.as_path(), day) else {
                    println!("Skipping day {day:02}");
                    continue;
                };

                let solution_func = solutions[(day - 1) as usize];
                criterion.bench_function(&format!("Day {day:02}"), |bench| {
                    bench.iter(|| solution_func(&input))
                });
            }

            criterion.final_summary();
        }
    }

    ExitCode::SUCCESS
}

pub fn get_action() -> Result<(Action, PathBuf, Vec<u8>), GetActionError> {
    let mut args = args().skip(1);

    let Some(action) = args.next() else {
        return Err(GetActionError::MissingRequiredArgument);
    };

    let action = match action.as_str() {
        "solve" => Action::Solve,
        "bench" => Action::Benchmark,
        _ => return Err(GetActionError::UnsupportedAction),
    };

    let Some(folder) = args.next().map(PathBuf::from) else {
        return Err(GetActionError::MissingRequiredArgument);
    };

    let mut days = Vec::new();
    let Some(arg) = args.next() else {
        let mut days = Vec::new();

        days.extend(1..=25);
        return Ok((action, folder, days));
    };

    for day in arg.split(',') {
        let Some(day) = day.parse::<u8>().ok() else {
            return Err(GetActionError::MisformattedDays);
        };

        if day > 25 {
            return Err(GetActionError::DayOutOfRange(day));
        }

        days.push(day);
    }

    if args.next().is_some() {
        return Err(GetActionError::MissingRequiredArgument);
    }

    Ok((action, folder, days))
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GetActionError {
    MissingRequiredArgument,
    UnsupportedAction,
    MisformattedDays,
    DayOutOfRange(u8),
}

impl core::fmt::Display for GetActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Usage: aoc2024 <ACTION> <INPUT_FOLDER> [DAYS]")?;

        match self {
            Self::MissingRequiredArgument => Ok(()),
            Self::UnsupportedAction => {
                writeln!(f)?;
                writeln!(
                    f,
                    "Unsupported action: `solve` and `bench` are the supported actions"
                )
            }
            Self::MisformattedDays => {
                writeln!(f)?;
                writeln!(f, "Ill-formatted [DAYS] format: 'day,day'")
            }
            Self::DayOutOfRange(day) => {
                writeln!(f)?;
                writeln!(f, "Provided day {day} is out of range")
            }
        }
    }
}

pub enum Action {
    Solve,
    Benchmark,
}

pub fn acquire_input(folder: &Path, day: u8) -> Option<String> {
    let mut folder = folder.to_owned();
    folder.push(format!("day_{day:02}.txt"));

    fs::read_to_string(folder).ok()
}

pub fn unimplemented(_: &str) -> (usize, usize) {
    todo!()
}

pub fn solve(
    input: &str,
    function: fn(&str) -> (usize, usize),
) -> (time::Duration, (usize, usize)) {
    let start = time::Instant::now();

    let result = function(input);

    let end = time::Instant::now();
    let elapsed = end.duration_since(start);

    (elapsed, result)
}
