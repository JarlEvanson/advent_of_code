use utils::Solution;

pub fn solve(input: &str) -> Solution {
    let lines = input.lines();
    let locations = lines.map(|line| {
        let mut numbers = line.split_ascii_whitespace();

        (
            numbers.next().unwrap().parse::<isize>().unwrap(),
            numbers.next().unwrap().parse::<isize>().unwrap(),
        )
    });

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    for (location_1, location_2) in locations {
        list_1.push(location_1);
        list_2.push(location_2);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let part_1 = list_1
        .iter()
        .copied()
        .zip(list_2.iter().copied())
        .map(|(id_1, id_2)| (id_1 - id_2).abs())
        .sum::<isize>() as usize;

    let mut list_2 = list_2.into_iter().peekable();

    let mut last = -1;
    let mut count = 0;

    let mut part_2 = 0;
    for id_1 in list_1.into_iter() {
        if id_1 != last {
            count = 0;

            while let Some(&id_2) = list_2.peek() {
                if id_2 > id_1 {
                    break;
                }

                if id_2 == id_1 {
                    count += 1;
                }

                let _ = list_2.next();
            }
        }

        last = id_1;
        part_2 += id_1 * count;
    }

    (part_1, part_2 as usize).into()
}
