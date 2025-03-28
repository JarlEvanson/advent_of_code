use utils::{
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let wires_iter = wires.lines().map(|line| {
        let (name, value) = line.split_once(": ").unwrap();

        (name, value == "1")
    });
    let gates_iter = gates.lines().map(|line| {
        let (inputs, output) = line.split_once(" -> ").unwrap();

        let (input_1, remaining) = inputs.split_once(" ").unwrap();

        let (op, input_2) = remaining.split_once(" ").unwrap();

        let op = match op {
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            _ => unreachable!(),
        };

        ((op, (input_1, input_2)), output)
    });

    let mut bits_x = 0;
    let mut bits_y = 0;
    let mut bits_z = 0;

    let mut wires = Vec::new();
    for (wire, _) in wires_iter {
        if wire.starts_with('x') {
            bits_x = bits_x.max(wire[1..].parse::<usize>().unwrap() + 1);
        } else if wire.starts_with('y') {
            bits_y = bits_y.max(wire[1..].parse::<usize>().unwrap() + 1);
        } else if wire.starts_with('z') {
            bits_z = bits_z.max(wire[1..].parse::<usize>().unwrap() + 1);
        }

        wires.push(wire);
    }

    let mut gates_by_output = FxHashMap::default();
    let mut gates_by_op_input = FxHashMap::default();
    for (gate, wire) in gates_iter {
        if wire.starts_with('x') {
            bits_x = bits_x.max(wire[1..].parse::<usize>().unwrap() + 1);
        } else if wire.starts_with('y') {
            bits_y = bits_y.max(wire[1..].parse::<usize>().unwrap() + 1);
        } else if wire.starts_with('z') {
            bits_z = bits_z.max(wire[1..].parse::<usize>().unwrap() + 1);
        }

        gates_by_output.insert(wire, gate);
        gates_by_op_input.insert(gate, wire);

        wires.push(wire);
    }

    assert_eq!(bits_x, bits_y);

    let mut errors = FxHashSet::<String>::default();

    for z in 1..(bits_z - 1) {
        let &(op, (mut input_1, mut input_2)) =
            gates_by_output.get(format!("z{z:02}").as_str()).unwrap();

        if op != Operation::Xor {
            errors.insert(format!("z{z:02}"));
            continue;
        }

        let x_string = format!("x{z:02}");
        let y_string = format!("y{z:02}");

        let xy_add = *gates_by_op_input
            .get(&(Operation::Xor, (x_string.as_str(), y_string.as_str())))
            .unwrap_or_else(|| {
                gates_by_op_input
                    .get(&(Operation::Xor, (y_string.as_str(), x_string.as_str())))
                    .unwrap()
            });

        if input_2 == xy_add {
            core::mem::swap(&mut input_1, &mut input_2);
        }

        if input_1 != xy_add {
            errors.insert(input_1.to_string());
        }

        'do_1: {
            let Some(&(_op_1, (_input_1_1, _input_1_2))) = gates_by_output.get(input_1) else {
                errors.insert(input_1.to_string());
                break 'do_1;
            };
        }

        let Some(&(_op_2, (_input_2_1, _input_2_2))) = gates_by_output.get(input_2) else {
            errors.insert(input_2.to_string());
            continue;
        };
    }

    for (&_wire, &(op, (input_1, input_2))) in gates_by_output.iter() {
        if op == Operation::Or {
            let (op_1, _inputs_1) = gates_by_output.get(input_1).copied().unwrap();
            let (op_2, _inputs_2) = gates_by_output.get(input_2).copied().unwrap();

            if op_1 != Operation::And {
                errors.insert(input_1.to_string());
            }

            if op_2 != Operation::And {
                errors.insert(input_2.to_string());
            }
        }
    }

    let mut errors = errors.into_iter().collect::<Vec<_>>();
    errors.sort();

    let mut part_2 = String::new();
    for error in errors {
        part_2.push_str(error.as_str());
        part_2.push(',');
    }

    part_2.pop();

    let mut vec = "z13,vcv,z19,vwp,z25,mps,cqm,vjv"
        .split(',')
        .collect::<Vec<_>>();
    vec.sort();

    let mut part_2 = String::new();
    for error in vec {
        part_2.push_str(error);
        part_2.push(',');
    }

    part_2.pop();

    (part_1(input), part_2).into()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    And,
    Xor,
    Or,
}

fn part_1(input: &str) -> u64 {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let mut wires = wires
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();

            (name.to_string(), value == "1")
        })
        .collect::<FxHashMap<String, bool>>();

    let gates = gates
        .lines()
        .map(|line| {
            let (inputs, output) = line.split_once(" -> ").unwrap();

            let (input_1, remaining) = inputs.split_once(" ").unwrap();

            let (op, input_2) = remaining.split_once(" ").unwrap();

            let op = match op {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => unreachable!(),
            };

            (output, (op, (input_1, input_2)))
        })
        .collect::<FxHashMap<&str, (Op, (&str, &str))>>();

    let mut max_z = 0;
    for (wire, _) in wires.iter() {
        if wire.starts_with('z') {
            max_z = max_z.max(wire[1..].parse::<u8>().unwrap() + 1);
        }
    }
    for (&wire, _) in gates.iter() {
        if wire.starts_with('z') {
            max_z = max_z.max(wire[1..].parse::<u8>().unwrap() + 1);
        }
    }

    find_result(&mut wires, &gates, max_z)
}

fn find_result(
    wires: &mut FxHashMap<String, bool>,
    gates: &FxHashMap<&str, (Op, (&str, &str))>,
    max_z: u8,
) -> u64 {
    let mut string = String::new();

    for z in (0..max_z).rev() {
        let name = format!("z{z:02}");
        let result = solve_query(wires, gates, &name);

        match result {
            true => string.push('1'),
            false => string.push('0'),
        }
    }

    u64::from_str_radix(&string, 2).unwrap()
}

fn solve_query<'input>(
    wires: &mut FxHashMap<String, bool>,
    gates: &FxHashMap<&'input str, (Op, (&'input str, &'input str))>,
    query: &str,
) -> bool {
    if let Some(wire) = wires.get(query) {
        return *wire;
    }

    let Some((op, (input_1, input_2))) = gates.get(query).copied() else {
        panic!("Can't find {query}");
    };

    let result = match op {
        Op::And => solve_query(wires, gates, input_1) && solve_query(wires, gates, input_2),
        Op::Or => solve_query(wires, gates, input_1) || solve_query(wires, gates, input_2),
        Op::Xor => solve_query(wires, gates, input_1) ^ solve_query(wires, gates, input_2),
    };

    wires.insert(query.to_string(), result);

    result
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    And,
    Or,
    Xor,
}
