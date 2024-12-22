use utils::{hash::FxHashMap, Solution};

pub fn solve(input: &str) -> Solution {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.lines();

    /*

    let mut part_1 = 0;
    let mut part_2 = 0;
    'design_loop: for design in designs {
        let mut found = false;

        let mut tries: Vec<(usize, &str)> = Vec::new();

        let mut check_start = 0;
        let mut try_index = 0;

        'find_loop: loop {
            while try_index < patterns.len() {
                let pattern = patterns[try_index];
                if design[check_start..].starts_with(pattern) {
                    check_start += pattern.len();
                    tries.push((try_index, pattern));
                    try_index = 0;

                    if check_start == design.len() {
                        if !found {
                            part_1 += 1;
                            found = true;
                        }

                        part_2 += 1;
                    }

                    continue 'find_loop;
                }

                try_index += 1;
            }

            if let Some((last_index, pattern)) = tries.pop() {
                check_start -= pattern.len();
                try_index = last_index + 1;
            } else {
                continue 'design_loop;
            }
        }
    }
    */

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut cache = FxHashMap::default();
    for design in designs {
        let count = combinations(design, &patterns, &mut cache);

        part_1 += (count > 0) as usize;
        part_2 += count;
    }

    (part_1, part_2).into()
}

pub fn combinations(
    design: &str,
    patterns: &Vec<&str>,
    seen: &mut FxHashMap<String, usize>,
) -> usize {
    if let Some(prev) = seen.get(design) {
        return *prev;
    }

    let from_here = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| {
            let (_, remaining) = design.split_at(pattern.len());
            if remaining.len() == 0 {
                1
            } else {
                combinations(remaining, patterns, seen)
            }
        })
        .sum();

    seen.insert(design.to_string(), from_here);

    from_here
}
