use utils::{
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let secrets = input.lines().map(|line| line.parse::<u64>().unwrap());

    let mut part_1 = 0u64;
    for secret in secrets.clone() {
        let mut secret = secret.clone();

        for _ in 0..2000 {
            secret = calculate_secret(secret);
        }

        part_1 += secret;
    }

    let mut sequences_count = FxHashMap::<[i64; 4], u64>::default();
    let mut set = FxHashSet::default();
    for secret in secrets {
        set.clear();

        let mut secrets = [0; 2001];
        secrets[0] = secret;

        for index in 1..secrets.len() {
            secrets[index] = calculate_secret(secrets[index - 1]);
        }

        let sequences = secrets.into_iter().map_windows(|&[a, b, c, d, e]| {
            (
                [
                    (b % 10) as i64 - (a % 10) as i64,
                    (c % 10) as i64 - (b % 10) as i64,
                    (d % 10) as i64 - (c % 10) as i64,
                    (e % 10) as i64 - (d % 10) as i64,
                ],
                e % 10,
            )
        });

        for (sequence, price) in sequences {
            if !set.insert(sequence) {
                continue;
            }

            *sequences_count.entry(sequence).or_insert(0) += price;
        }
    }

    let part_2 = sequences_count.into_values().max().unwrap();

    (part_1, part_2).into()
}

fn calculate_secret(secret: u64) -> u64 {
    let step_0 = prune(mix(secret, secret * 64));

    let step_1 = prune(mix(step_0, step_0 / 32));

    let step_2 = prune(mix(step_1, step_1 * 2048));

    step_2
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
