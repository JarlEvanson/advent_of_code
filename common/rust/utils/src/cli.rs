use std::{
    env::args,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{self, Duration},
};

use criterion::Criterion;

use crate::Solution;

pub fn run(solutions: [Option<fn(&str) -> Solution>; 25]) -> ExitCode {
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
                let Some(solution) = solutions[(day - 1) as usize] else {
                    println!("No solver: Skipping day {day:02}");
                    continue;
                };

                let Some(input) = acquire_input(input_folder.as_path(), day) else {
                    println!("No input: Skipping day {day:02}");
                    continue;
                };

                let (duration, solution) = solve(&input, solution);
                total_time += duration;

                println!("Day {day:02}: {solution} Time: {duration:?}");
            }

            println!("Total Processing Time: {total_time:?}");
        }
        Action::Benchmark => {
            let mut criterion = Criterion::default();

            for day in days {
                let Some(solution) = solutions[(day - 1) as usize] else {
                    println!("No solver: Skipping day {day:02}");
                    continue;
                };

                let Some(input) = acquire_input(input_folder.as_path(), day) else {
                    println!("No input: Skipping day {day:02}");
                    continue;
                };

                criterion.bench_function(&format!("Day {day:02}"), |bench| {
                    bench.iter(|| solution(&input))
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
        return Ok((action, folder, Vec::from_iter(1..=25)));
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

pub fn solve(input: &str, function: fn(&str) -> Solution) -> (time::Duration, Solution) {
    let start = time::Instant::now();

    let result = function(input);

    let end = time::Instant::now();
    let elapsed = end.duration_since(start);

    (elapsed, result)
}
