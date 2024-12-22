use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap());

    let a = registers.next().unwrap();
    let b = registers.next().unwrap();
    let c = registers.next().unwrap();

    let instruction_string = program.trim().split_once(": ").unwrap().1;

    let instructions = instruction_string
        .split(',')
        .map(|instruction| instruction.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part_1 = run_computer([a, b, c], &instructions);

    let part_2 = solve_2(&instructions, instructions.len() - 1, 0).unwrap();

    (part_1, part_2).into()
}

fn solve_2(instructions: &[usize], cursor: usize, possible: usize) -> Option<usize> {
    for candidate in 0..8 {
        if run_computer([possible * 8 + candidate, 0, 0], instructions)
            .split(',')
            .map(|number| number.parse::<usize>().unwrap())
            .zip(instructions.iter().copied().skip(cursor))
            .all(|value| value.0 == value.1)
        {
            if cursor == 0 {
                return Some(possible * 8 + candidate);
            } else if let Some(solution) =
                solve_2(instructions, cursor - 1, possible * 8 + candidate)
            {
                return Some(solution);
            }
        }
    }

    return None;
}

pub fn run_computer(registers: [usize; 3], instructions: &[usize]) -> String {
    use core::fmt::Write;

    let [mut a, mut b, mut c] = registers;

    let mut ip = 0;

    let mut outputted = false;
    let mut output_string = String::new();
    while let Some((next_instruction, next_data)) = instructions
        .get(ip)
        .copied()
        .zip(instructions.get(ip + 1).copied())
    {
        let combo_operand = match next_data {
            0..=3 => next_data,
            4 => a,
            5 => b,
            6 => c,
            7 => unreachable!(),
            _ => unreachable!(),
        };

        match next_instruction {
            0 => a = a / 2usize.pow(combo_operand as u32),
            1 => b = b ^ next_data,
            2 => b = combo_operand % 8,
            3 => {
                if a != 0 {
                    ip = next_data;
                    continue;
                }
            }
            4 => b = b ^ c,
            5 => {
                if outputted {
                    output_string.push(',');
                }
                write!(output_string, "{}", combo_operand % 8).unwrap();
                outputted = true;
            }
            6 => b = a / 2usize.pow(combo_operand as u32),
            7 => c = a / 2usize.pow(combo_operand as u32),
            _ => unreachable!(),
        }

        ip += 2;
    }

    output_string
}
